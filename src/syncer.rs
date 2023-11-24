use futures::future::join_all;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::sync::Arc;
use std::{env, time::Instant};
use tokio::{
    task::JoinHandle,
    time::{sleep, Duration},
};
use tracing::{info, instrument, log::LevelFilter};

mod assets;
mod bribes;
mod gauges;
mod killed_gauges;
mod pair_aprs;
mod pairs;
mod types;

use assets::{update_assets_from_tokenlist, update_other_db_assets_prices};
use pairs::update_pairs;
use types::Chain;

use killed_gauges::update_killed_gauges;

#[instrument]
pub async fn syncer() {
    let db_url = env::var("DATABASE_URL").expect("Should be defined in .env");
    let mut opt = ConnectOptions::new(db_url);
    opt.sqlx_logging_level(LevelFilter::Trace);

    let iteration_time_secs = env::var("ITERATION_TIME_SECS")
        .expect("Should be defined in .env")
        .parse::<u64>()
        .expect("Should be a number");

    // set up connection
    let conn = Database::connect(opt)
        .await
        .expect("Could not connect to database");

    let conn: Arc<_> = Arc::new(conn);

    let fantom_rpc_url = env::var("FANTOM_RPC_URL")
        .expect("Should be defined in .env")
        .to_string();
    let fantom_chain = Chain::new(fantom_rpc_url, 250);

    let base_rpc_url = env::var("BASE_RPC_URL")
        .expect("Should be defined in .env")
        .to_string();
    let base_chain = Chain::new(base_rpc_url, 8453);

    let canto_rpc_url = env::var("CANTO_RPC_URL")
        .expect("Should be defined in .env")
        .to_string();
    let canto_chain = Chain::new(canto_rpc_url, 7700);

    let mantle_rpc_url = env::var("MANTLE_RPC_URL")
        .expect("Should be defined in .env")
        .to_string();
    let mantle_chain = Chain::new(mantle_rpc_url, 5000);

    let chains = vec![fantom_chain, base_chain, canto_chain, mantle_chain];

    iteration_run(chains.clone(), Arc::clone(&conn)).await;

    let six_hours = Duration::from_secs(iteration_time_secs);
    loop {
        info!("Sleeping for {} seconds", six_hours.as_secs());
        sleep(six_hours).await;
        iteration_run(chains.clone(), Arc::clone(&conn)).await;
    }
}

async fn iteration_run(chains: Vec<Chain>, conn: Arc<DatabaseConnection>) {
    let now = Instant::now();
    let tasks: Vec<JoinHandle<()>> = chains
        .into_iter()
        .map(|chain| {
            let pool = Arc::clone(&conn);
            let task = tokio::spawn(async move {
                update_assets_from_tokenlist(&chain, &pool).await.unwrap();
                update_other_db_assets_prices(&chain, &pool).await.unwrap();
                update_pairs(&chain, &pool).await.unwrap();
                update_killed_gauges(chain, pool).await.unwrap();
            });
            task
        })
        .collect();
    join_all(tasks).await;
    info!("Iteration took {} seconds", now.elapsed().as_secs());
}
