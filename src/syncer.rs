use futures::future::join_all;
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{info, instrument};

mod assets;
mod bribes;
mod gauges;
mod killed_gauges;
mod pair_aprs;
mod pairs;
mod types;

use assets::update_assets_from_tokenlist;
use pairs::update_pairs;
use types::Chain;

use killed_gauges::update_killed_gauges;

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

    let fantom_rpc_url = env::var("FANTOM_RPC_URL")
        .expect("Should be defined in .env")
        .to_string();
    let fantom_chain: Chain = Chain::new(fantom_rpc_url, 250);

    let base_rpc_url = env::var("BASE_RPC_URL")
        .expect("Should be defined in .env")
        .to_string();
    let base_chain: Chain = Chain::new(base_rpc_url, 8453);

    let canto_rpc_url = env::var("CANTO_RPC_URL")
        .expect("Should be defined in .env")
        .to_string();
    let canto_chain: Chain = Chain::new(canto_rpc_url, 7700);

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
