use axum::{extract::Path, http::StatusCode, response::Json, Extension};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter};

use backend::database::aprs::Entity as Aprs;
use backend::database::assets::{Column as AssetsColumn, Entity as Assets, Model as Asset};
use backend::database::bribes::Entity as Bribes;
use backend::database::gauges::Entity as Gauges;
use backend::database::killed_gauges::Entity as KilledGauges;
use backend::database::pairs::{Column as PairsColumn, Entity as Pairs};

use crate::server::{
    internal_error,
    types::{PairResponse, PairsResponse},
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
    let pairs = Pairs::find()
        .filter(PairsColumn::ChainId.eq(chain_id))
        .all(&conn)
        .await
        .map_err(internal_error)?;

    let gauges = pairs
        .load_many(Gauges, &conn)
        .await
        .map_err(internal_error)?;

    let aprs = pairs.load_many(Aprs, &conn).await.map_err(internal_error)?;

    let bribes = pairs
        .load_many(Bribes, &conn)
        .await
        .map_err(internal_error)?;

    let killed_gauges = pairs
        .load_many(KilledGauges, &conn)
        .await
        .map_err(internal_error)?;

    let pairs_res = pairs
        .into_iter()
        .zip(gauges.into_iter())
        .zip(aprs.into_iter())
        .zip(bribes.into_iter())
        .zip(killed_gauges.into_iter())
        .map(
            |((((pair, gauges), aprs), bribes), killed_gauges)| PairResponse {
                pair,
                gauge: gauges.first().cloned(),
                aprs,
                bribes,
                killed_gauges,
            },
        )
        .collect();

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
