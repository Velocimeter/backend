use serde::{Deserialize, Serialize};

use backend::database::{
    aprs::Model as AprsModel, bribes::Model as BribesModel, gauges::Model as GaugeModel,
    killed_gauges::Model as KilledGaugeModel, pairs::Model as PairModel,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct PairsResponse {
    pub pairs: Vec<PairResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PairResponse {
    pub pair: PairModel,
    pub gauge: Option<GaugeModel>,
    pub aprs: Vec<AprsModel>,
    pub bribes: Vec<BribesModel>,
    pub killed_gauges: Vec<KilledGaugeModel>,
}