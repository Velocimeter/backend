use axum::http::StatusCode;
use ethers::{
    abi::Address,
    contract::{abigen, ContractCall, Multicall},
    prelude::{Http, Provider},
    types::{H160, U256},
    utils::{format_ether, format_units, to_checksum},
};
use eyre::Result;
use sea_orm::{sea_query, ActiveValue, DatabaseConnection, EntityTrait};
use serde_json::json;
use std::sync::Arc;
use tracing::{error, info, instrument};

use backend::bindings::{CarbonPair, Factory, Voter};
use backend::database::pairs::{ActiveModel as ActivePair, Column as PairsColumn, Entity as Pairs};

use crate::server::internal_error;
use crate::syncer::{
    assets::find_asset,
    gauges::update_gauge,
    types::{Chain, PairType},
};

#[instrument(skip_all)]
pub async fn update_graphene_pairs(chain: &Chain, conn: &Arc<DatabaseConnection>) -> Result<()> {
    info!(
        "Collecting graphene pairs for chain id: {}",
        chain.get_chain_data().id
    );
    let provider = Provider::<Http>::try_from(chain.get_chain_data().rpc_url.to_string())?;
    let client = Arc::new(provider);

    let factory_address = chain
        .get_chain_data()
        .graphene_factory_address
        .parse::<Address>()?;

    if factory_address == Address::zero() {
        info!(
            "Graphene factory doesn't exist for chain id: {}",
            chain.get_chain_data().id
        );
        return Ok(());
    }

    let factory = Factory::new(factory_address, Arc::clone(&client));
    // TODO: fix this with new factory
    // let all_pairs = factory.all_pairs_length().call().await?;
    let all_pairs = U256::from(1);

    let calls: Vec<ContractCall<Provider<Http>, Address>> = (0..all_pairs.as_u64())
        .map(|i| factory.all_pairs(i.into()))
        .collect();

    let mut multicall = Multicall::<Provider<Http>>::new(
        client.clone(),
        Some(
            chain
                .get_chain_data()
                .multicall_address
                .parse::<Address>()
                .expect("Address is set by hand"),
        ),
    )
    .await?;

    multicall.add_calls(false, calls);

    let pair_addresses: Vec<Address> = multicall.call_array().await?;

    for pair_address in pair_addresses {
        match update_pair(pair_address, chain.clone(), Arc::clone(&client), conn).await {
            Ok(_) => {}
            Err(e) => {
                info!("Error updating pair {}: {:?}", pair_address, e);
            }
        }
    }

    Ok(())
}

async fn update_pair(
    pair_address: H160,
    chain: Chain,
    client: Arc<Provider<Http>>,
    conn: &Arc<DatabaseConnection>,
) -> Result<()> {
    abigen!(
        CarbonController,
        r#"[
            pairTradingFeePPM(address, address) public view returns (uint256)
        ]"#,
    );

    let voter_address = chain.get_chain_data().voter_address.parse::<Address>()?;
    let voter = Voter::new(voter_address, Arc::clone(&client));
    let pair = CarbonPair::new(pair_address, Arc::clone(&client));
    let mut multicall = Multicall::<Provider<Http>>::new(
        client.clone(),
        Some(
            chain
                .get_chain_data()
                .multicall_address
                .parse::<Address>()
                .expect("Address is set by hand"),
        ),
    )
    .await?;

    multicall.add_calls(
        false,
        [
            pair.balance_of_token_0(),
            pair.balance_of_token_1(),
            pair.total_supply(),
        ],
    );
    multicall.add_calls(
        false,
        [pair.token_0(), pair.token_1(), voter.gauges(pair_address)],
    );
    multicall.add_call(pair.symbol(), false);

    let (reserve0, reserve1, total_supply, token_0, token_1, gauge, symbol) = multicall
        .call::<(U256, U256, U256, H160, H160, H160, String)>()
        .await?;

    let fee: U256 = CarbonController::new(
        chain
            .get_chain_data()
            .carbon_controller_address
            .parse::<Address>()
            .expect("Set by hand"),
        Arc::clone(&client),
    )
    .pair_trading_fee_ppm(token_0, token_1)
    .call()
    .await?;

    let token_0 = find_asset(to_checksum(&token_0, None), &chain, conn).await?;
    let token_1 = find_asset(to_checksum(&token_1, None), &chain, conn).await?;

    let reserve0 = format_units(reserve0, token_0.decimals)
        .expect("Should not happen")
        .parse::<f64>()?;
    let reserve1 = format_units(reserve1, token_1.decimals)
        .expect("Should not happen")
        .parse::<f64>()?;
    let total_supply = format_ether(total_supply).parse::<f64>()?;
    let fee = fee.as_u64() as f64 / 10000.0;
    let tvl = get_tvl(reserve0, reserve1, token_0.price, token_1.price);

    let pair_address_checksumed = to_checksum(&pair_address, None);

    let pair = ActivePair {
        pair_type: ActiveValue::Set(PairType::GrapheneCL.as_str()),
        chain_id: ActiveValue::Set(chain.get_chain_data().id),
        decimals: ActiveValue::Set(18),
        address: ActiveValue::Set(pair_address_checksumed.to_owned()),
        gauge_address: ActiveValue::Set(to_checksum(&gauge, None)),
        symbol: ActiveValue::Set(symbol),
        fee: ActiveValue::Set(fee),
        token0_address: ActiveValue::Set(token_0.address.to_lowercase()),
        token0: ActiveValue::Set(json!(token_0)),
        token1_address: ActiveValue::Set(token_1.address.to_lowercase()),
        token1: ActiveValue::Set(json!(token_1)),
        reserve0: ActiveValue::Set(reserve0),
        reserve1: ActiveValue::Set(reserve1),
        total_supply: ActiveValue::Set(total_supply),
        tvl: ActiveValue::Set(tvl),
        stable: ActiveValue::Set(false),
    };

    match write_pair(conn, pair_address_checksumed, pair).await {
        Ok(_) => {}
        Err(e) => {
            error!("Error writing to DB: {:?}", e);
        }
    }

    if gauge != Address::zero() {
        info!("Pair {} is a gauge", pair_address);
        update_gauge(pair_address, gauge, tvl, &chain, Arc::clone(&client), conn).await?;
    } else {
        info!("Pair {} is not a gauge", pair_address);
    }

    Ok(())
}

fn get_tvl(reserve0: f64, reserve1: f64, token0_price: f64, token1_price: f64) -> f64 {
    let mut tvl = 0.0;

    if token0_price != 0.0 {
        tvl += reserve0 * token0_price;
    }

    if token1_price != 0.0 {
        tvl += reserve1 * token1_price;
    }

    if tvl != 0.0 && (token0_price == 0.0 || token1_price == 0.0) {
        tvl = tvl * 2.0;
    }

    tvl
}

async fn write_pair(
    conn: &Arc<DatabaseConnection>,
    pair_address: String,
    pair: ActivePair,
) -> Result<(), StatusCode> {
    match Pairs::insert(pair)
        .on_conflict(
            sea_query::OnConflict::columns([PairsColumn::Address, PairsColumn::ChainId])
                .update_columns([
                    PairsColumn::GaugeAddress,
                    PairsColumn::Reserve0,
                    PairsColumn::Reserve1,
                    PairsColumn::TotalSupply,
                    PairsColumn::Tvl,
                    PairsColumn::Token0,
                    PairsColumn::Token1,
                    PairsColumn::Fee,
                ])
                .to_owned(),
        )
        .exec(conn.as_ref())
        .await
        .map_err(internal_error)
    {
        Ok(_) => {}
        Err(e) => {
            error!("Error writing to DB: {:?}, pair {}", e, pair_address);
            return Err(e);
        }
    }
    info!("Pair {} DB write successful", pair_address);
    Ok(())
}
