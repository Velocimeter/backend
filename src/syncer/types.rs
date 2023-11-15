use backend::config::*;
use ethers::types::Address;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ChainData {
    pub id: i32,
    pub name: String,
    pub geckoterminal_name: String,
    pub rpc_url: String,
    pub native_gauge_address: String,
    pub router_address: String,
    pub factory_address: String,
    pub voter_address: String,
    pub minter_address: String,
    pub ve_address: String,
    pub rewards_dist_address: String,
    pub tokenlists_url: String,
    pub default_token_address: String,
    pub o_token_address: String,
    pub stablecoin_address: String,
    pub route_token_address: String,
    pub wblt_address: String,
    pub multicall_address: String,
}

#[derive(Debug, Clone)]
pub enum Chain {
    Fantom(ChainData),
    Base(ChainData),
    Canto(ChainData),
}

impl Chain {
    pub fn new(rpc_url: String, chain_id: i32) -> Self {
        match chain_id {
            250 => Self::Fantom(ChainData {
                id: 250,
                rpc_url,
                name: String::from("Fantom"),
                geckoterminal_name: String::from("ftm"),
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
                wblt_address: format!("{:?}", Address::zero()),
                native_gauge_address: FANTOM_GAUGE.to_string(),
                multicall_address: MULTICALL_ADDRESS.to_string(),
            }),
            8453 => Self::Base(ChainData {
                id: 8453,
                rpc_url,
                name: String::from("Base"),
                geckoterminal_name: String::from("base"),
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
                wblt_address: BASE_WBLT.to_string(),
                native_gauge_address: BASE_GAUGE.to_string(),
                multicall_address: MULTICALL_ADDRESS.to_string(),
            }),
            7700 => Self::Canto(ChainData {
                id: 7700,
                rpc_url,
                name: String::from("Canto"),
                geckoterminal_name: String::from("canto"),
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
                wblt_address: format!("{:?}", Address::zero()),
                native_gauge_address: CANTO_GAUGE.to_string(),
                multicall_address: MULTICALL_ADDRESS.to_string(),
            }),
            _ => panic!("Chain id not supported"),
        }
    }
    pub fn get_chain_data(&self) -> &ChainData {
        match self {
            Chain::Fantom(data) => data,
            Chain::Base(data) => data,
            Chain::Canto(data) => data,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Asset {
    pub address: String,
    pub chainId: i32,
    pub decimals: i32,
    pub logoURI: String,
    pub name: String,
    pub symbol: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct AssetWithPrice {
    pub address: String,
    pub chainId: i32,
    pub decimals: i32,
    pub logoURI: String,
    pub name: String,
    pub symbol: String,
    pub price: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum GeckoTerminalResponse {
    Success(GeckoTerminalSuccessResponse),
    Error(GeckoTerminalFailedResponse),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeckoTerminalSuccessResponse {
    pub data: GeckoTerminalData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeckoTerminalFailedResponse {
    pub errors: Vec<GeckoterminalError>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeckoTerminalData {
    id: String,
    pub attributes: GeckoTerminalAttributes,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeckoterminalError {
    pub status: String,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeckoTerminalAttributes {
    name: String,
    address: String,
    symbol: String,
    decimals: i32,
    total_supply: String,
    coingecko_coin_id: String,
    pub price_usd: String,
    fdv_usd: String,
    total_reserve_in_usd: String,
    volume_usd: serde_json::Value,
    market_cap_usd: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DexscreenerResponse {
    pub pairs: Vec<DexscreenerPair>,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct DexscreenerPair {
    pub chainId: String,
    pub dexId: String,
    pub url: String,
    pub pairAddress: String,
    pub baseToken: DexscreenerToken,
    pub quoteToken: DexscreenerToken,
    pub priceNative: String,
    pub priceUsd: Option<String>,
    pub txns: DexscreenerTxns,
    pub volume: DexscreenerVolume,
    pub priceChange: DexscreenerPriceChange,
    pub liquidity: Option<DexscreenerLiquidity>,
    pub fdv: Option<f64>,
    pub pairCreatedAt: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DexscreenerToken {
    pub address: String,
    pub name: String,
    pub symbol: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DexscreenerTxns {
    pub m5: DexscreenerTxnsData,
    pub h1: DexscreenerTxnsData,
    pub h6: DexscreenerTxnsData,
    pub h24: DexscreenerTxnsData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DexscreenerTxnsData {
    pub buys: i32,
    pub sells: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DexscreenerVolume {
    pub m5: f64,
    pub h1: f64,
    pub h6: f64,
    pub h24: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DexscreenerPriceChange {
    pub m5: f64,
    pub h1: f64,
    pub h6: f64,
    pub h24: f64,
}

#[derive(Debug, Deserialize, Serialize)]

pub struct DexscreenerLiquidity {
    pub usd: Option<f64>,
    pub base: f64,
    pub quote: f64,
}
