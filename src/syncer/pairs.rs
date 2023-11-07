use axum::http::StatusCode;
use ethers::{
    abi::Address,
    contract::Multicall,
    prelude::{Http, Provider},
    types::{H160, U256},
    utils::{format_ether, format_units, to_checksum},
};
use eyre::Result;
use sea_orm::{sea_query, ActiveValue, DatabaseConnection, EntityTrait};
use std::sync::Arc;
use tracing::{error, info, instrument};

use backend::bindings::{Factory, Pair, Voter};
use backend::database::pairs::{ActiveModel as ActivePair, Column as PairsColumn, Entity as Pairs};

use crate::server::internal_error;
use crate::syncer::{assets::find_asset, gauges::update_gauge, types::Chain};

#[instrument(skip(chain, conn))]
pub async fn update_pairs(chain: Chain, conn: Arc<DatabaseConnection>) -> Result<()> {
    info!(
        "Collecting pairs for chain id: {}",
        chain.get_chain_data().id
    );
    let provider = Provider::<Http>::try_from(chain.get_chain_data().rpc_url.to_string())?;
    let client = Arc::new(provider);

    let factory_address = chain.get_chain_data().factory_address.parse::<Address>()?;
    let factory = Factory::new(factory_address, Arc::clone(&client));
    let all_pairs = factory.all_pairs_length().call().await?;
    let mut pair_addresses: Vec<H160> = vec![];

    for i in 0..all_pairs.as_u64() {
        let pair_address = factory.all_pairs(i.into()).call().await?;
        pair_addresses.push(pair_address);
    }

    for pair_address in pair_addresses {
        match update_pair(chain.clone(), pair_address, Arc::clone(&client), &conn).await {
            Ok(_) => {}
            Err(e) => {
                info!("Error updating pair {}: {:?}", pair_address, e);
            }
        }
    }

    Ok(())
}

#[instrument(skip(chain, client, conn))]
async fn update_pair(
    chain: Chain,
    pair_address: H160,
    client: Arc<Provider<Http>>,
    conn: &Arc<DatabaseConnection>,
) -> Result<()> {
    let voter_address = chain.get_chain_data().voter_address.parse::<Address>()?;
    let voter = Voter::new(voter_address, Arc::clone(&client));
    let pair = Pair::new(pair_address, Arc::clone(&client));
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

    multicall.add_call(pair.get_reserves(), false);
    multicall.add_calls(
        false,
        [pair.token_0(), pair.token_1(), voter.gauges(pair_address)],
    );
    multicall.add_call(pair.total_supply(), false);
    multicall.add_call(pair.symbol(), false);
    multicall.add_call(pair.stable(), false);

    let ((reserve0, reserve1, _timestamp), token_0, token_1, gauge, total_supply, symbol, stable) =
        multicall
            .call::<((U256, U256, U256), H160, H160, H160, U256, String, bool)>()
            .await?;

    let token_0 = find_asset(to_checksum(&token_0, None), &chain, conn).await?;
    let token_1 = find_asset(to_checksum(&token_1, None), &chain, conn).await?;

    let reserve0 = format_units(reserve0, token_0.decimals)
        .unwrap()
        .parse::<f64>()?;
    let reserve1 = format_units(reserve1, token_1.decimals)
        .unwrap()
        .parse::<f64>()?;
    let total_supply = format_ether(total_supply).parse::<f64>()?;

    let tvl = get_tvl(reserve0, reserve1, token_0.price, token_1.price);

    let pair = ActivePair {
        chain_id: ActiveValue::Set(chain.get_chain_data().id),
        decimals: ActiveValue::Set(18),
        address: ActiveValue::Set(to_checksum(&pair_address, None)),
        gauge_address: ActiveValue::Set(to_checksum(&gauge, None)),
        symbol: ActiveValue::Set(symbol),
        token0_address: ActiveValue::Set(token_0.address),
        token1_address: ActiveValue::Set(token_1.address),
        reserve0: ActiveValue::Set(reserve0),
        reserve1: ActiveValue::Set(reserve1),
        total_supply: ActiveValue::Set(total_supply),
        tvl: ActiveValue::Set(tvl),
        stable: ActiveValue::Set(stable),
    };

    match write_pair(conn, pair).await {
        Ok(_) => {}
        Err(e) => {
            error!("Error writing to DB: {:?}", e);
        }
    }

    if gauge != Address::zero() {
        info!("Pair {} is a gauge", pair_address);
        update_gauge(pair_address, gauge, tvl, &chain, Arc::clone(&client), &conn).await?;
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

#[instrument(skip(conn))]
async fn write_pair(conn: &Arc<DatabaseConnection>, pair: ActivePair) -> Result<(), StatusCode> {
    match Pairs::insert(pair)
        .on_conflict(
            sea_query::OnConflict::columns([PairsColumn::Address, PairsColumn::ChainId])
                .update_columns([
                    PairsColumn::GaugeAddress,
                    PairsColumn::Reserve0,
                    PairsColumn::Reserve1,
                    PairsColumn::TotalSupply,
                    PairsColumn::Tvl,
                ])
                .to_owned(),
        )
        .exec(conn.as_ref())
        .await
        .map_err(internal_error)
    {
        Ok(_) => {}
        Err(e) => {
            error!("Error writing to DB: {:?}", e);
            return Err(e);
        }
    }
    info!("DB write successful");
    Ok(())
}
