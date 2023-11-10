use axum::{extract::Path, http::StatusCode, response::Json, Extension};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use backend::database::aprs::Entity as Aprs;
use backend::database::assets::{Column as AssetsColumn, Entity as Assets, Model as Asset};
use backend::database::bribes::{Column as BribesColumn, Entity as Bribes};
use backend::database::gauges::Entity as Gauges;
use backend::database::killed_gauges::Entity as KilledGauges;
use backend::database::pairs::{Column as PairsColumn, Entity as Pairs};

use crate::server::{
    internal_error,
    types::{BribeResponse, PairResponse, PairsResponse},
};

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}

#[axum_macros::debug_handler]
pub async fn give_pairs(
    Path(chain_id): Path<i32>,
    Extension(conn): Extension<DatabaseConnection>,
) -> Result<Json<PairsResponse>, StatusCode> {
    let mut pairs_res: Vec<PairResponse> = vec![];

    let pairs = Pairs::find()
        .filter(PairsColumn::ChainId.eq(chain_id))
        .find_with_related(Aprs)
        .all(&conn)
        .await
        .map_err(internal_error)?;

    for (pair, aprs) in pairs.into_iter() {
        let gauge = Gauges::find_by_id((pair.gauge_address.to_owned(), chain_id))
            .one(&conn)
            .await
            .map_err(internal_error)?;
        let bribes = Bribes::find()
            .filter(
                BribesColumn::ChainId
                    .eq(chain_id)
                    .and(BribesColumn::PairAddress.eq(&pair.address)),
            )
            .find_also_related(Assets)
            .all(&conn)
            .await
            .map_err(internal_error)?;

        let bribes_with_tokens = bribes
            .into_iter()
            .map(|(bribe, token)| BribeResponse { bribe, token })
            .collect();

        let killed_gauges = KilledGauges::find_by_id((pair.address.to_owned(), chain_id))
            .all(&conn)
            .await
            .map_err(internal_error)?;

        let pair = PairResponse {
            pair,
            gauge,
            bribes: bribes_with_tokens,
            aprs,
            killed_gauges,
        };

        pairs_res.push(pair);
    }

    let res = PairsResponse { pairs: pairs_res };

    Ok(Json(res))
}

#[axum_macros::debug_handler]
pub async fn give_assets(
    Path(chain_id): Path<i32>,
    Extension(conn): Extension<DatabaseConnection>,
) -> Result<Json<Vec<Asset>>, StatusCode> {
    let res = Assets::find()
        .filter(AssetsColumn::ChainId.eq(chain_id))
        .all(&conn)
        .await
        .map_err(internal_error)?;

    Ok(Json(res))
}

#[axum_macros::debug_handler]
pub async fn give_asset(
    Path((chain_id, address)): Path<(i32, String)>,
    Extension(conn): Extension<DatabaseConnection>,
) -> Result<Json<Asset>, StatusCode> {
    let res = Assets::find()
        .filter(
            AssetsColumn::ChainId
                .eq(chain_id)
                .and(AssetsColumn::Address.eq(address.to_lowercase())),
        )
        .one(&conn)
        .await
        .map_err(internal_error)?;

    match res {
        Some(asset) => Ok(Json(asset)),
        None => Err(StatusCode::NOT_FOUND),
    }
}
