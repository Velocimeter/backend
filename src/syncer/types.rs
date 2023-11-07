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
    pub multicall_address: String,
}

#[derive(Debug, Clone)]
pub enum Chain {
    Fantom(ChainData),
    Base(ChainData),
    Canto(ChainData),
}

impl Chain {
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
pub struct GeckoTerminalResponse {
    pub data: GeckoTerminalData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeckoTerminalData {
    id: String,
    pub attributes: GeckoTerminalAttributes,
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
    pub fdv: Option<String>,
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
    pub m5: String,
    pub h1: String,
    pub h6: String,
    pub h24: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DexscreenerPriceChange {
    pub m5: String,
    pub h1: String,
    pub h6: String,
    pub h24: String,
}

#[derive(Debug, Deserialize, Serialize)]

pub struct DexscreenerLiquidity {
    pub usd: Option<String>,
    pub base: String,
    pub quote: String,
}
