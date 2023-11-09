use axum::http::StatusCode;
use ethers::{
    contract::Multicall,
    providers::{Http, Provider},
    types::{Address, H160, U256},
    utils::{format_ether, format_units, to_checksum},
};
use eyre::Result;
use sea_orm::{sea_query, ActiveValue, DatabaseConnection, EntityTrait};
use std::sync::Arc;
use tracing::{error, info, instrument};

use crate::server::internal_error;
use crate::syncer::types::Chain;
use crate::syncer::{assets::find_asset, bribes::update_bribe, pair_aprs::update_pair_aprs};
use backend::bindings::{Gauge, Minter, Voter, VotingEscrow};
use backend::database::gauges::{
    ActiveModel as ActiveGauge, Column as GaugesColumn, Entity as Gauges,
};

#[instrument(skip(chain, client, conn))]
pub async fn update_gauge(
    pair_address: H160,
    gauge_address: H160,
    tvl: f64,
    chain: &Chain,
    client: Arc<Provider<Http>>,
    conn: &Arc<DatabaseConnection>,
) -> Result<()> {
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

    let voter_address = chain.get_chain_data().voter_address.parse::<Address>()?;
    let default_token_address = chain
        .get_chain_data()
        .default_token_address
        .parse::<Address>()?;
    let gauge = Gauge::new(gauge_address, client.clone());
    let voter = Voter::new(voter_address, client.clone());

    multicall.add_call(gauge.total_supply(), false);
    multicall.add_call(gauge.reward_rate(default_token_address), false);
    multicall.add_call(voter.external_bribes(gauge_address), false);
    multicall.add_call(voter.weights(pair_address), false);

    let (total_supply, reward_rate, bribe_address, votes) =
        multicall.call::<(U256, U256, H160, U256)>().await?;

    let total_supply = format_ether(total_supply).parse::<f64>()?;
    let reward = format_units(reward_rate, 18)?.parse::<f64>()? * 86400.0; // 86400 seconds in a day
    let votes = format_units(votes, 18)?.parse::<f64>()?;

    let mut max_tbv = 0.0;
    let mut median_tbv = 0.0;
    let mut min_tbv = 0.0;

    if bribe_address != H160::zero() {
        (min_tbv, median_tbv, max_tbv) = update_bribe(
            pair_address,
            bribe_address,
            chain,
            Arc::clone(&client),
            conn,
        )
        .await?;
    }

    let (min_apr, max_apr) =
        match update_gauge_aprs(min_tbv, max_tbv, votes, chain, Arc::clone(&client), conn).await {
            Ok((min_apr, max_apr)) => (min_apr, max_apr),
            Err(e) => {
                info!("Error updating gauge APRs: {:?}", e);
                (0.0, 0.0)
            }
        };

    let gauge = ActiveGauge {
        address: ActiveValue::set(to_checksum(&gauge_address, None)),
        pair_address: ActiveValue::set(to_checksum(&pair_address, None)),
        chain_id: ActiveValue::set(chain.get_chain_data().id),
        decimals: ActiveValue::set(18),
        total_supply: ActiveValue::set(total_supply),
        reward: ActiveValue::set(reward),
        bribe_address: ActiveValue::set(to_checksum(&bribe_address, None)),
        votes: ActiveValue::set(votes),
        max_tbv: ActiveValue::set(max_tbv),
        median_tbv: ActiveValue::set(median_tbv),
        min_tbv: ActiveValue::set(min_tbv),
        max_apr: ActiveValue::set(max_apr),
        min_apr: ActiveValue::set(min_apr),
    };

    update_pair_aprs(pair_address, gauge_address, tvl, chain, client, conn).await?;

    match write_gauge(conn, gauge).await {
        Ok(_) => {}
        Err(e) => {
            error!("Error writing to DB: {:?}", e);
        }
    }

    Ok(())
}

async fn update_gauge_aprs(
    min_tbv: f64,
    max_tbv: f64,
    votes: f64,
    chain: &Chain,
    client: Arc<Provider<Http>>,
    conn: &Arc<DatabaseConnection>,
) -> Result<(f64, f64)> {
    let minter_address = chain
        .get_chain_data()
        .minter_address
        .parse::<Address>()
        .expect("Address is set by hand");
    let ve_address = chain
        .get_chain_data()
        .ve_address
        .parse::<Address>()
        .expect("Address is set by hand");
    let minter = Minter::new(minter_address, Arc::clone(&client));
    let ve = VotingEscrow::new(ve_address, Arc::clone(&client));

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

    multicall.add_calls(false, [minter.weekly_emission(), ve.supply()]);

    let (weekly, supply) = multicall.call::<(U256, U256)>().await?;

    let growth = minter.calculate_growth(weekly).call().await.unwrap();

    if supply == U256::zero() {
        return Ok((0.0, 0.0));
    }
    let rebase_apr = ((growth * U256::from(52)) / supply) * U256::from(100);
    let rebase_apr = rebase_apr.as_u64() as f64;

    let mut max_apr = rebase_apr;
    let mut min_apr = rebase_apr;

    let token = find_asset(
        chain.get_chain_data().default_token_address.to_string(),
        chain,
        conn,
    )
    .await?;

    if token.price * votes > 0.0 {
        max_apr = max_tbv * 52.0 / (token.price * votes) * 100.0;
        min_apr = min_tbv * 52.0 / (token.price * votes) * 100.0;
    }

    Ok((min_apr, max_apr))
}

#[instrument(skip(conn))]
async fn write_gauge(conn: &Arc<DatabaseConnection>, gauge: ActiveGauge) -> Result<(), StatusCode> {
    match Gauges::insert(gauge)
        .on_conflict(
            sea_query::OnConflict::columns([GaugesColumn::Address, GaugesColumn::ChainId])
                .update_columns([
                    GaugesColumn::Votes,
                    GaugesColumn::TotalSupply,
                    GaugesColumn::MaxApr,
                    GaugesColumn::MinApr,
                    GaugesColumn::MedianTbv,
                    GaugesColumn::MinTbv,
                    GaugesColumn::MaxTbv,
                    GaugesColumn::Reward,
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
