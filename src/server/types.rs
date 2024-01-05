use serde::{Deserialize, Serialize};

use backend::database::{
    aprs::Model as AprsModel, bribes::Model as BribeModel, gauges::Model as GaugeModel,
    killed_gauges::Model as KilledGaugeModel, pairs::Model as PairModel,
};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct PairsResponse {
    pub pairs: Vec<PairResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PairResponse {
    pub address: String,
    pub chain_id: i32,
    pub pair_type: String,
    pub symbol: String,
    pub decimals: i32,
    pub stable: bool,
    pub fee: f64,
    pub total_supply: f64,
    pub reserve0: f64,
    pub reserve1: f64,
    pub gauge_address: String,
    pub tvl: f64,
    pub token0_address: String,
    pub token1_address: String,
    pub token0: Value,
    pub token1: Value,
    pub gauge: Option<GaugeModel>,
    pub aprs: Option<Vec<AprsModel>>,
    pub bribes: Option<Vec<BribeModel>>,
    pub killed_gauges: Vec<KilledGaugeModel>,
}

impl PairResponse {
    pub fn new(
        pair: PairModel,
        gauge: Option<GaugeModel>,
        aprs: Vec<AprsModel>,
        bribes: Vec<BribeModel>,
        killed_gauges: Vec<KilledGaugeModel>,
    ) -> Self {
        Self {
            address: pair.address,
            chain_id: pair.chain_id,
            pair_type: pair.pair_type,
            symbol: pair.symbol,
            decimals: pair.decimals,
            stable: pair.stable,
            fee: pair.fee,
            total_supply: pair.total_supply,
            reserve0: pair.reserve0,
            reserve1: pair.reserve1,
            gauge_address: pair.gauge_address,
            tvl: pair.tvl,
            token0_address: pair.token0_address,
            token1_address: pair.token1_address,
            token0: pair.token0,
            token1: pair.token1,
            gauge,
            aprs: aprs.len().gt(&0).then_some(aprs),
            bribes: bribes.len().gt(&0).then_some(bribes),
            killed_gauges,
        }
    }
}
