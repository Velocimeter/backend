use axum::http::StatusCode;
use ethers::{
    contract::Multicall,
    providers::{Http, Provider},
    types::{Address, H160, U256},
    utils::{format_units, to_checksum},
};
use eyre::Result;
use sea_orm::{
    sea_query, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter,
};
use serde_json::json;
use std::sync::Arc;
use tracing::{error, info};

use crate::server::internal_error;
use crate::syncer::types::Chain;
use backend::bindings::Bribe;
use backend::database::bribes::{
    ActiveModel as ActiveBribe, Column as BribesColumn, Entity as Bribes,
};

use crate::syncer::assets::{check_if_token_is_option, check_option_ve_discount, find_asset};

///
/// Update the bribe for a given bribe address and write to DB.
/// Expected to return
/// ```
/// (min_tbv, median_tbv, max_tbv)
/// ```
///
pub async fn update_bribe(
    pair_address: H160,
    bribe_address: H160,
    chain: &Chain,
    client: Arc<Provider<Http>>,
    conn: &Arc<DatabaseConnection>,
) -> Result<(f64, f64, f64)> {
    let mut min_tbv = 0.0;
    let mut max_tbv = 0.0;

    let bribe = Bribe::new(bribe_address, Arc::clone(&client));
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

    let tokens_len = bribe.rewards_list_length().call().await?;
    let mut bribe_tokens = vec![];

    let mut calls = vec![];
    for i in 0..tokens_len.as_u64() {
        let bribe_token_address = bribe.rewards(U256::from(i)).call().await?;
        bribe_tokens.push(bribe_token_address);
        let call = bribe.left(bribe_token_address);
        calls.push(call);
    }

    multicall.add_calls(false, calls);

    let left_for_bribes = multicall.call_array::<U256>().await?;

    clean_up_stale_bribes(bribe_address, bribe_tokens.clone(), conn).await?;

    for (bribe_token_address, left) in bribe_tokens.into_iter().zip(left_for_bribes.into_iter()) {
        let bribe_token_address = to_checksum(&bribe_token_address, None);
        let token = find_asset(bribe_token_address.to_string(), chain, conn).await?;

        let (is_option, underlying_address) =
            check_if_token_is_option(&bribe_token_address, Arc::clone(&client)).await?;

        if is_option {
            let underlying_address = to_checksum(&underlying_address, None);
            let underlying_token = find_asset(underlying_address, chain, conn).await?;
            let ve_discount =
                check_option_ve_discount(&bribe_token_address, Arc::clone(&client)).await?;
            let max_token_value = underlying_token.price * (100.0 - ve_discount) / 100.0;
            if token.price > 0.0 && max_token_value > 0.0 {
                max_tbv += format_units(left, token.decimals)?.parse::<f64>()? * max_token_value;
                min_tbv += format_units(left, token.decimals)?.parse::<f64>()? * token.price;
            }
        } else if token.price > 0.0 {
            max_tbv += format_units(left, token.decimals)?.parse::<f64>()? * token.price;
            min_tbv += format_units(left, token.decimals)?.parse::<f64>()? * token.price;
        }

        let reward_amount = format_units(left, token.decimals)?.parse::<f64>()?;

        let bribe_address = to_checksum(&bribe_address, None);

        let bribe = ActiveBribe {
            bribe_address: ActiveValue::Set(bribe_address.to_owned()),
            token_address: ActiveValue::Set(bribe_token_address.to_lowercase()),
            token: ActiveValue::Set(json!(token)),
            pair_address: ActiveValue::Set(to_checksum(&pair_address, None)),
            chain_id: ActiveValue::Set(chain.get_chain_data().id),
            reward_amount: ActiveValue::Set(reward_amount),
        };

        match write_bribe(conn, bribe_address, bribe).await {
            Ok(_) => {}
            Err(e) => {
                error!("Error writing to DB: {:?}", e);
            }
        }
    }

    let median_tbv = (min_tbv + max_tbv) / 2.0;

    Ok((min_tbv, median_tbv, max_tbv))
}

///
/// If DB has bribes that are not longer in list of reward tokens in bribe contract, remove them.
///
async fn clean_up_stale_bribes(
    bribe_address: H160,
    bribe_tokens: Vec<Address>,
    conn: &Arc<DatabaseConnection>,
) -> Result<()> {
    let bribe_address = to_checksum(&bribe_address, None);
    let bribes = Bribes::find()
        .filter(BribesColumn::BribeAddress.eq(&bribe_address))
        .all(conn.as_ref())
        .await?;

    if bribes.is_empty() {
        return Ok(());
    }

    let bribe_tokens = bribe_tokens
        .into_iter()
        .map(|t_a| to_checksum(&t_a, None).to_lowercase())
        .collect::<Vec<String>>();

    for bribe in bribes {
        if !bribe_tokens.contains(&bribe.token_address.to_lowercase()) {
            let b_a = bribe.bribe_address.clone();
            let t_a = bribe.token_address.clone();
            let delete_result = bribe.delete(conn.as_ref()).await;
            match delete_result {
                Ok(_) => {
                    info!("Deleted stale bribe entry: b_a {}, t_a{}", b_a, t_a);
                }
                Err(e) => {
                    error!("Error deleting stale bribe entry: {:?}, address {}", e, b_a);
                }
            }
        }
    }

    Ok(())
}

///
/// Write bribe to DB.
///
async fn write_bribe(
    conn: &Arc<DatabaseConnection>,
    bribe_address: String,
    bribe: ActiveBribe,
) -> Result<(), StatusCode> {
    match Bribes::insert(bribe)
        .on_conflict(
            sea_query::OnConflict::columns([
                BribesColumn::BribeAddress,
                BribesColumn::TokenAddress,
                BribesColumn::PairAddress,
            ])
            .update_columns([BribesColumn::RewardAmount, BribesColumn::Token])
            .to_owned(),
        )
        .exec(conn.as_ref())
        .await
        .map_err(internal_error)
    {
        Ok(_) => {}
        Err(e) => {
            error!("Error writing bribe {} to DB: {:?}", bribe_address, e);
            return Err(e);
        }
    }
    info!("Bribe {} DB write successful", bribe_address);
    Ok(())
}
