use axum::http::StatusCode;
use ethers::{
    contract::Multicall,
    prelude::{Http, Provider},
    types::{Address, H160, U256},
    utils::to_checksum,
};
use eyre::Result;
use sea_orm::{sea_query, ActiveValue, DatabaseConnection, DbErr, EntityTrait};
use std::sync::Arc;
use tracing::{error, info, instrument};

use backend::bindings::{Gauge, Voter};
use backend::database::killed_gauges::{
    ActiveModel as ActiveKilledGauge, Column as KilledGaugesColumn, Entity as KilledGauges,
};

use crate::syncer::types::Chain;

#[instrument(skip_all)]
pub async fn update_killed_gauges(chain: Chain, conn: Arc<DatabaseConnection>) -> Result<()> {
    info!(
        "Collecting killed gauges for chain id: {}",
        chain.get_chain_data().id
    );
    let provider = Provider::<Http>::try_from(chain.get_chain_data().rpc_url.to_string())?;
    let client = Arc::new(provider);

    let voter_address = chain.get_chain_data().voter_address.parse::<Address>()?;
    let voter = Voter::new(voter_address, Arc::clone(&client));

    let dead_gauges_count = voter.killed_gauges_length().call().await?;

    if dead_gauges_count == U256::zero() {
        info!("No dead gauges found");
        return Ok(());
    }

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
        (0..dead_gauges_count.as_u64()).map(|i| voter.killed_gauges(U256::from(i))),
    );

    let dead_gauges_addresses = multicall.call_array::<H160>().await?;

    multicall.clear_calls();

    multicall.add_calls(
        false,
        dead_gauges_addresses.clone().into_iter().map(|g_a| {
            let gauge = Gauge::new(g_a, Arc::clone(&client));
            gauge.stake()
        }),
    );

    let pair_addresses_of_killed_gauges = multicall.call_array::<H160>().await?;

    let killed_gauges = dead_gauges_addresses
        .into_iter()
        .zip(pair_addresses_of_killed_gauges.into_iter())
        .map(|(dead_gauge, pair)| ActiveKilledGauge {
            address: ActiveValue::set(to_checksum(&dead_gauge, None)),
            pair_address: ActiveValue::set(to_checksum(&pair, None)),
            chain_id: ActiveValue::set(chain.get_chain_data().id),
            decimals: ActiveValue::set(18),
            bribe_address: ActiveValue::set(to_checksum(&Address::zero(), None)),
            total_supply: ActiveValue::set(0.0),
            reward: ActiveValue::set(0.0),
            max_apr: ActiveValue::set(0.0),
            max_tbv: ActiveValue::set(0.0),
            median_tbv: ActiveValue::set(0.0),
            min_apr: ActiveValue::set(0.0),
            min_tbv: ActiveValue::set(0.0),
            votes: ActiveValue::set(0.0),
            bribes: ActiveValue::set(None),
            rewards: ActiveValue::set(None),
        })
        .collect::<Vec<ActiveKilledGauge>>();

    write_killed_gauges(killed_gauges, conn)
        .await
        .expect("Error writing killed gauges to DB");

    Ok(())
}

async fn write_killed_gauges(
    killed_gauges: Vec<ActiveKilledGauge>,
    conn: Arc<DatabaseConnection>,
) -> Result<(), StatusCode> {
    for killed_gauge in killed_gauges {
        write_killed_gauge(&conn, killed_gauge).await?;
    }
    Ok(())
}

async fn write_killed_gauge(
    conn: &Arc<DatabaseConnection>,
    killed_gauge: ActiveKilledGauge,
) -> Result<(), StatusCode> {
    match KilledGauges::insert(killed_gauge)
        .on_conflict(
            sea_query::OnConflict::columns([
                KilledGaugesColumn::Address,
                KilledGaugesColumn::ChainId,
            ])
            .do_nothing()
            .to_owned(),
        )
        .exec(conn.as_ref())
        .await
    {
        Ok(_) => {}
        Err(e) => match e {
            DbErr::RecordNotInserted => {
                info!("Killed gauge already exists");
            }
            _ => {
                error!("Error writing killed gauge to DB: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        },
    }
    info!("Killed gauges DB write successful");
    Ok(())
}
