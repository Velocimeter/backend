use futures::future::join_all;
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{info, instrument};

mod assets;
mod bribes;
mod config;
mod gauges;
mod killed_gauges;
mod pair_aprs;
mod pairs;
mod types;

use assets::update_assets_from_tokenlist;
use config::*;
use pairs::update_pairs;
use types::Chain;

use self::killed_gauges::update_killed_gauges;
use crate::syncer::types::ChainData;

#[instrument]
pub async fn syncer() {
    let db_url = env::var("DATABASE_URL").expect("Should be defined in .env");
    let iteration_time_secs = env::var("ITERATION_TIME_SECS")
        .expect("Should be defined in .env")
        .parse::<u64>()
        .expect("Should be a number");

    // set up connection
    let conn = Database::connect(db_url)
        .await
        .expect("Could not connect to database");

    let conn: Arc<_> = Arc::new(conn);

    let fantom_chain: Chain = Chain::Fantom(ChainData {
        id: 250,
        name: String::from("Fantom"),
        geckoterminal_name: String::from("ftm"),
        rpc_url: env::var("FANTOM_RPC_URL")
            .expect("Should be defined in .env")
            .to_string(),
        router_address: FANTOM_ROUTER.to_string(),
        factory_address: FANTOM_FACTORY.to_string(),
        voter_address: FANTOM_VOTER.to_string(),
        ve_address: FANTOM_VE.to_string(),
        minter_address: FANTOM_MINTER.to_string(),
        rewards_dist_address: FANTOM_REWARDS_DIST.to_string(),
        tokenlists_url: FANTOM_TOKENLISTS.to_string(),
        default_token_address: FANTOM_DEFAULT_TOKEN.to_string(),
        o_token_address: FANTOM_O_TOKEN.to_string(),
        stablecoin_address: FANTOM_STABLECOIN.to_string(),
        route_token_address: FANTOM_ROUTE_TOKEN.to_string(),
        native_gauge_address: FANTOM_GAUGE.to_string(),
        multicall_address: MULTICALL_ADDRESS.to_string(),
    });

    let base_chain: Chain = Chain::Base(ChainData {
        id: 8453,
        name: String::from("Base"),
        geckoterminal_name: String::from("base"),
        rpc_url: env::var("BASE_RPC_URL")
            .expect("Should be defined in .env")
            .to_string(),
        router_address: BASE_ROUTER.to_string(),
        factory_address: BASE_FACTORY.to_string(),
        voter_address: BASE_VOTER.to_string(),
        ve_address: BASE_VE.to_string(),
        minter_address: BASE_MINTER.to_string(),
        rewards_dist_address: BASE_REWARDS_DIST.to_string(),
        tokenlists_url: BASE_TOKENLISTS.to_string(),
        default_token_address: BASE_DEFAULT_TOKEN.to_string(),
        o_token_address: BASE_O_TOKEN.to_string(),
        stablecoin_address: BASE_STABLECOIN.to_string(),
        route_token_address: BASE_ROUTE_TOKEN.to_string(),
        native_gauge_address: BASE_GAUGE.to_string(),
        multicall_address: MULTICALL_ADDRESS.to_string(),
    });

    let canto_chain: Chain = Chain::Canto(ChainData {
        id: 7700,
        name: String::from("Canto"),
        geckoterminal_name: String::from("canto"),
        rpc_url: env::var("CANTO_RPC_URL")
            .expect("Should be defined in .env")
            .to_string(),
        router_address: CANTO_ROUTER.to_string(),
        factory_address: CANTO_FACTORY.to_string(),
        voter_address: CANTO_VOTER.to_string(),
        ve_address: CANTO_VE.to_string(),
        minter_address: CANTO_MINTER.to_string(),
        rewards_dist_address: CANTO_REWARDS_DIST.to_string(),
        tokenlists_url: CANTO_TOKENLISTS.to_string(),
        default_token_address: CANTO_DEFAULT_TOKEN.to_string(),
        o_token_address: CANTO_O_TOKEN.to_string(),
        stablecoin_address: CANTO_STABLECOIN.to_string(),
        route_token_address: CANTO_ROUTE_TOKEN.to_string(),
        native_gauge_address: CANTO_GAUGE.to_string(),
        multicall_address: MULTICALL_ADDRESS.to_string(),
    });

    let chains = vec![fantom_chain, base_chain, canto_chain];

    iteration_run(chains.clone(), Arc::clone(&conn)).await;

    let six_hours = Duration::from_secs(iteration_time_secs);
    loop {
        info!("Sleeping for {} seconds", six_hours.as_secs());
        sleep(six_hours).await;
        iteration_run(chains.clone(), Arc::clone(&conn)).await;
    }
}

async fn iteration_run(chains: Vec<Chain>, conn: Arc<DatabaseConnection>) {
    let mut tasks = vec![];
    for chain in chains {
        let pool = Arc::clone(&conn);
        let task = tokio::spawn(async move {
            update_assets_from_tokenlist(&chain, &pool).await.unwrap();
            update_pairs(&chain, &pool).await.unwrap();
            update_killed_gauges(chain, pool).await.unwrap();
        });
        tasks.push(task);
    }
    join_all(tasks).await;
}
