use async_recursion::async_recursion;
use axum::http::StatusCode;
use backend::bindings::{oTOKEN, Router, ERC20};
use backend::database::assets::{
    ActiveModel as ActiveAsset, Column as AssetColumn, Column as AssetsColumn, Entity as Assets,
};
use ethers::contract::abigen;
use ethers::types::{H160, U256};
use ethers::utils::{format_units, parse_units, to_checksum};
use ethers::{
    abi::Address,
    contract::Multicall,
    prelude::{Http, Provider},
};
use eyre::Result;
use sea_orm::{sea_query, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::sync::Arc;
use tracing::{error, info, instrument};

use crate::server::internal_error;
use crate::syncer::types::AssetWithPrice;
use crate::syncer::types::{
    Asset, Chain, DexscreenerPair, DexscreenerResponse, GeckoTerminalResponse,
};

///
/// Return asset from DB if exists, otherwise update asset, save to DB and return.
///
pub async fn find_asset(
    address: String,
    chain: &Chain,
    conn: &Arc<DatabaseConnection>,
) -> Result<AssetWithPrice> {
    let chain_id = chain.get_chain_data().id;
    let asset = Assets::find_by_id((address.to_string().to_lowercase(), chain_id))
        .one(conn.as_ref())
        .await?;
    match asset {
        Some(asset) => {
            let asset = AssetWithPrice {
                address: asset.address,
                chainId: asset.chain_id,
                decimals: asset.decimals,
                logoURI: asset.logo_url,
                name: asset.name,
                symbol: asset.symbol,
                price: asset.price,
            };
            Ok(asset)
        }
        None => {
            let asset = update_asset(&address, chain, conn).await?;
            Ok(asset)
        }
    }
}

///
/// Update all assets for a given chain and tokenlist. Save to DB.
///
#[instrument(skip_all)]
pub async fn update_assets_from_tokenlist(
    chain: &Chain,
    conn: &Arc<DatabaseConnection>,
) -> Result<()> {
    info!("Updating assets: {} chain id", chain.get_chain_data().id);
    let http_client = reqwest::Client::builder().build()?;
    let provider = Provider::<Http>::try_from(chain.get_chain_data().rpc_url.to_string())?;
    let client = Arc::new(provider);

    let token_list = &chain.get_chain_data().tokenlists_url;
    let res = http_client.get(token_list).send().await?;
    let res = res.json::<Vec<Asset>>().await?;
    info!("Assets from tokenlist: {}", res.len());
    for asset in res {
        let chain_id = chain.get_chain_data().id;
        let client = Arc::clone(&client);
        let price = update_asset_price(&asset.address, asset.decimals, chain, client, conn).await?;
        let active_asset = ActiveAsset {
            address: ActiveValue::set(asset.address.to_string().to_lowercase()),
            chain_id: ActiveValue::set(chain_id),
            decimals: ActiveValue::set(asset.decimals),
            logo_url: ActiveValue::set(asset.logoURI.to_string()),
            name: ActiveValue::set(asset.name.to_string()),
            symbol: ActiveValue::set(asset.symbol.to_string()),
            price: ActiveValue::set(price),
        };
        write_asset(conn, asset.address, active_asset)
            .await
            .unwrap();
    }

    Ok(())
}

///
/// Update assets that has been already written to db, but need a price update for next iteration. Tokens from the tokenlist get updated anyway. Save to DB.
///
#[instrument(skip_all)]
pub async fn update_other_db_assets_prices(
    chain: &Chain,
    conn: &Arc<DatabaseConnection>,
) -> Result<()> {
    let chain_id = chain.get_chain_data().id;
    info!("Updating other db assets prices: {} chain id", chain_id);

    let http_client = reqwest::Client::builder().build()?;
    let token_list = &chain.get_chain_data().tokenlists_url;
    let res = http_client.get(token_list).send().await?;
    let tokenlist_assets_addresses: Vec<String> = res
        .json::<Vec<Asset>>()
        .await?
        .into_iter()
        .map(|asset| asset.address.to_lowercase())
        .collect();
    let res = Assets::find()
        .filter(
            AssetsColumn::ChainId
                .eq(chain_id)
                .and(AssetColumn::Address.is_not_in(tokenlist_assets_addresses)),
        )
        .all(conn.as_ref())
        .await?;

    info!("Other assets from db: {}", res.len());
    if res.is_empty() {
        return Ok(());
    }

    let provider = Provider::<Http>::try_from(chain.get_chain_data().rpc_url.to_string())?;
    let client = Arc::new(provider);

    for asset in res {
        let chain_id = chain.get_chain_data().id;
        let client = Arc::clone(&client);
        let price = update_asset_price(&asset.address, asset.decimals, chain, client, conn).await?;
        let active_asset = ActiveAsset {
            address: ActiveValue::set(asset.address.to_string().to_lowercase()),
            chain_id: ActiveValue::set(chain_id),
            decimals: ActiveValue::set(asset.decimals),
            logo_url: ActiveValue::set(asset.logo_url.to_string()),
            name: ActiveValue::set(asset.name.to_string()),
            symbol: ActiveValue::set(asset.symbol.to_string()),
            price: ActiveValue::set(price),
        };
        write_asset(conn, asset.address, active_asset)
            .await
            .unwrap();
    }

    Ok(())
}

///
/// Update asset for a given address and chain, save to DB and return.
///
#[async_recursion]
pub async fn update_asset(
    address: &String,
    chain: &Chain,
    conn: &Arc<DatabaseConnection>,
) -> Result<AssetWithPrice> {
    info!("Updating asset: {:?}", address);
    let provider = Provider::<Http>::try_from(chain.get_chain_data().rpc_url.to_string())?;
    let client = Arc::new(provider);
    let chain_id = chain.get_chain_data().id;

    let token_contract = ERC20::new(address.parse::<Address>()?, client.clone());
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

    multicall.add_calls(false, [token_contract.name(), token_contract.symbol()]);
    multicall.add_call(token_contract.decimals(), false);

    let (name, symbol, decimals) = multicall.call::<(String, String, u8)>().await?;

    let price = update_asset_price(address, decimals.into(), chain, client, conn).await?;

    let asset = ActiveAsset {
        address: ActiveValue::set(address.to_string().to_lowercase()),
        chain_id: ActiveValue::set(chain_id),
        decimals: ActiveValue::set(decimals.into()),
        logo_url: ActiveValue::set("".to_string()),
        name: ActiveValue::set(name.to_string()),
        symbol: ActiveValue::set(symbol.to_string()),
        price: ActiveValue::set(price),
    };
    write_asset(conn, address.to_owned(), asset).await.unwrap();

    let asset = AssetWithPrice {
        address: address.to_string(),
        chainId: chain_id,
        decimals: decimals.into(),
        logoURI: "".to_string(),
        name: name.to_string(),
        symbol: symbol.to_string(),
        price,
    };

    Ok(asset)
}

///
/// Update asset price for a given address and chain, return price.
/// If asset is an option, return discounted price "in market".
///
async fn update_asset_price(
    address: &String,
    decimals: i32,
    chain: &Chain,
    client: Arc<Provider<Http>>,
    conn: &Arc<DatabaseConnection>,
) -> Result<f64> {
    // wBLT integration price check
    if chain.get_chain_data().id == 8453
        && chain.get_chain_data().wblt_address.to_lowercase() == address.to_lowercase()
    {
        let price = get_wblt_price(Arc::clone(&client)).await;
        if let Ok(price) = price {
            return Ok(price);
        }
    }

    let (is_option, underlying_address) =
        check_if_token_is_option(address, Arc::clone(&client)).await?;
    if is_option {
        let price = get_asset_price(
            &to_checksum(&underlying_address, None),
            decimals,
            chain,
            Arc::clone(&client),
            conn,
        )
        .await?;
        let discount = check_option_discount(address, Arc::clone(&client)).await?;
        let price = price * (100.0 - discount) / 100.0;
        return Ok(price);
    }

    let price = get_asset_price(address, decimals, chain, client, conn).await?;
    Ok(price)
}

///
/// Get asset price for a given address and chain, return price.
/// First check geckoterminal price.
/// Second check dexscreener price.
/// Third check aggregated price in ETH.
/// Fourth check aggregated price in stablecoin.
///
async fn get_asset_price(
    address: &String,
    decimals: i32,
    chain: &Chain,
    client: Arc<Provider<Http>>,
    conn: &Arc<DatabaseConnection>,
) -> Result<f64> {
    let stablecoin_address = &chain.get_chain_data().stablecoin_address;

    if address.to_lowercase() == stablecoin_address.to_lowercase() {
        return Ok(1.0);
    }

    let aggregated_in_stables = get_aggregated_price_in_stables(address, chain.clone()).await?;
    if aggregated_in_stables > 0.0 {
        return Ok(aggregated_in_stables);
    }
    let aggregated_in_stablecoin = get_aggregated_price_in_stablecoin(
        address,
        decimals,
        stablecoin_address,
        chain,
        Arc::clone(&client),
    )
    .await?;
    if aggregated_in_stablecoin > 0.0 {
        return Ok(aggregated_in_stablecoin);
    }
    let aggregated_in_eth =
        get_aggregated_price_in_eth(address, decimals, chain.clone(), Arc::clone(&client), conn)
            .await?;
    if aggregated_in_eth > 0.0 {
        return Ok(aggregated_in_eth);
    }
    // Base special case
    if chain.get_chain_data().id == 8453 {
        let price = get_aggregated_price_in_wblt(
            address,
            decimals,
            chain.clone(),
            Arc::clone(&client),
            conn,
        )
        .await?;
        if price > 0.0 {
            return Ok(price);
        }
    }
    // Mantle special case
    if chain.get_chain_data().id == 5000 {
        const FRAX_ADDRESS: &str = "0x406cde76a3fd20e48bc1e0f60651e60ae204b040";
        const SFRAX_ADDRESS: &str = "0xf3602c5a7f625181659445c8dddde73da15c22e3";
        // FRAX
        if address.to_lowercase() == FRAX_ADDRESS.to_lowercase() {
            return Ok(1.0);
        }
        // sFRAX
        if address.to_lowercase() == SFRAX_ADDRESS.to_lowercase() {
            return Ok(1.0);
        }
        // if paired with FRAX
        let aggregated_in_stablecoin = get_aggregated_price_in_stablecoin(
            address,
            decimals,
            FRAX_ADDRESS,
            chain,
            Arc::clone(&client),
        )
        .await?;
        if aggregated_in_stablecoin > 0.0 {
            return Ok(aggregated_in_stablecoin);
        }
        // if pairs with sFRAX
        let aggregated_in_stablecoin =
            get_aggregated_price_in_stablecoin(address, decimals, SFRAX_ADDRESS, chain, client)
                .await?;
        if aggregated_in_stablecoin > 0.0 {
            return Ok(aggregated_in_stablecoin);
        }
    }
    Ok(0.0)
}

///
/// Get aggregated price in stablecoins from geckoterminal or dexscreener.
///
async fn get_aggregated_price_in_stables(address: &String, chain: Chain) -> Result<f64> {
    let geckoterminal_name = &chain.get_chain_data().geckoterminal_name;
    let chain_name = &chain.get_chain_data().name;

    let price = geckoterminal(address, geckoterminal_name).await;
    if let Ok(price) = price {
        if price > 0.0 {
            return Ok(price);
        }
    }

    let price = dexscreener(address, chain_name).await;
    if let Ok(price) = price {
        return Ok(price);
    }

    Ok(0.0)
}

///
/// Get price from geckoterminal.
///
async fn geckoterminal(address: &String, chain_name: &String) -> Result<f64> {
    let url = format!(
        "https://api.geckoterminal.com/api/v2/networks/{}/tokens/{}",
        chain_name, address
    );
    let http_client = reqwest::Client::builder().build()?;
    let res = http_client.get(&url).send().await?;
    if let Ok(res_json) = res.json::<GeckoTerminalResponse>().await {
        match res_json {
            GeckoTerminalResponse::Success(res) => {
                let price = res
                    .data
                    .attributes
                    .price_usd
                    .unwrap_or_default()
                    .parse::<f64>()?;
                return Ok(price);
            }
            GeckoTerminalResponse::Error(_) => return Ok(0.0),
            GeckoTerminalResponse::RateLimited(_) => return Ok(0.0),
        }
    }
    Ok(0.0)
}

///
/// Get price from dexscreener.
///
async fn dexscreener(address: &String, chain_name: &str) -> Result<f64> {
    let url = format!("https://api.dexscreener.com/latest/dex/tokens/{}", address);
    let http_client = reqwest::Client::builder().build()?;
    let res = http_client.get(url).send().await?;
    let res = res.json::<DexscreenerResponse>().await?;
    let mut pairs = res.pairs;
    if pairs.is_empty() {
        return Ok(0.0);
    }

    pairs.sort_by(|a, b| {
        let a = a.txns.h24.buys + a.txns.h24.sells;
        let b = b.txns.h24.buys + b.txns.h24.sells;
        b.cmp(&a)
    });

    let filtered_pairs: Vec<DexscreenerPair> = pairs
        .into_iter()
        .filter(|pair| {
            pair.baseToken.address.to_lowercase() == address.to_string().to_lowercase()
                && pair
                    .liquidity
                    .as_ref()
                    .is_some_and(|liq| liq.usd.is_some_and(|liq_usd| liq_usd > 1000.0))
                && pair.chainId.to_lowercase() == chain_name.to_lowercase()
                // no multichain usdc/dai
                && pair.quoteToken.address.to_lowercase()
                    != "0x04068DA6C83AFCFA0e13ba15A6696662335D5B75".to_lowercase()
                && pair.quoteToken.address.to_lowercase()
                    != "0x8D11eC38a3EB5E956B052f67Da8Bdc9bef8Abf3E".to_lowercase()
        })
        .collect();

    if let Some(pair) = filtered_pairs.first() {
        let price = pair.priceUsd.clone().unwrap_or_default().parse::<f64>()?;
        return Ok(price);
    }

    Ok(0.0)
}

///
/// Get price using ETH price using Router contract. Returns zero if Contract Logic error.
///
async fn get_aggregated_price_in_eth(
    address: &str,
    decimals: i32,
    chain: Chain,
    client: Arc<Provider<Http>>,
    conn: &Arc<DatabaseConnection>,
) -> Result<f64> {
    let route_token_address = &chain.get_chain_data().route_token_address;
    let route_token_parsed_address = route_token_address.parse::<Address>()?;
    let route_token = find_asset(route_token_address.to_string(), &chain, conn).await?;

    let route_token_price = route_token.price;

    let router_address = &chain.get_chain_data().router_address;
    let router_address = router_address.parse::<Address>()?;

    let token_address = address.parse::<Address>()?;

    let router = Router::new(router_address, client);
    let amount_in = parse_units(1, decimals)?;
    let amount_out = router
        .get_amount_out(amount_in.into(), token_address, route_token_parsed_address)
        .call()
        .await;

    match amount_out {
        Ok(amount_out) => {
            let (amount_out, _is_stable) = amount_out;
            let amount_out = format_units(amount_out, 18)?.parse::<f64>()?;
            let price = amount_out * route_token_price;
            Ok(price)
        }
        Err(_) => Ok(0.0),
    }
}

///
/// Get price using stablecoin using Router contract. Returns zero if Contract Logic error.
///
async fn get_aggregated_price_in_stablecoin(
    address: &str,
    decimals: i32,
    stablecoin_address: &str,
    chain: &Chain,
    client: Arc<Provider<Http>>,
) -> Result<f64> {
    let stablecoin_address = stablecoin_address.parse::<Address>()?;
    let stablecoin_decimals_call = ERC20::new(stablecoin_address, Arc::clone(&client)).decimals();

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

    let router_address = &chain.get_chain_data().router_address;
    let router_address = router_address.parse::<Address>()?;

    let token_address = address.parse::<Address>()?;

    let router = Router::new(router_address, client);
    let amount_in = parse_units(1, decimals)?;
    let amount_out_call =
        router.get_amount_out(amount_in.into(), token_address, stablecoin_address);

    multicall.add_call(amount_out_call, false);
    multicall.add_call(stablecoin_decimals_call, false);

    let multicall_result = multicall.call::<((U256, bool), u8)>().await;

    match multicall_result {
        Ok((amount_out, stablecoin_decimals)) => {
            let (amount_out, _is_stable) = amount_out;
            let decimals: usize = stablecoin_decimals.into();
            let amount_out = format_units(amount_out, decimals)?.parse::<f64>()?;
            Ok(amount_out)
        }
        Err(_) => Ok(0.0),
    }
}

///
/// Get price using wBLT price using Router contract. Returns zero if Contract Logic error.
/// Part of wBLT integration.
///
async fn get_aggregated_price_in_wblt(
    address: &str,
    decimals: i32,
    chain: Chain,
    client: Arc<Provider<Http>>,
    conn: &Arc<DatabaseConnection>,
) -> Result<f64> {
    let wblt_token_address = &chain.get_chain_data().wblt_address;
    let wblt_token_parsed_address = wblt_token_address.parse::<Address>()?;
    let wblt_token = find_asset(wblt_token_address.to_string(), &chain, conn).await?;

    let wblt_token_price = wblt_token.price;

    let router_address = &chain.get_chain_data().router_address;
    let router_address = router_address.parse::<Address>()?;

    let token_address = address.parse::<Address>()?;

    let router = Router::new(router_address, client);
    let amount_in = parse_units(1, decimals)?;
    let amount_out = router
        .get_amount_out(amount_in.into(), token_address, wblt_token_parsed_address)
        .call()
        .await;

    match amount_out {
        Ok(amount_out) => {
            let (amount_out, _is_stable) = amount_out;
            let amount_out = format_units(amount_out, 18)?.parse::<f64>()?;
            let price = amount_out * wblt_token_price;
            Ok(price)
        }
        Err(_) => Ok(0.0),
    }
}

///
/// Check if token is an option. If it is, returns underlying address. Otherwise returns zero address.
///
pub async fn check_if_token_is_option(
    address: &str,
    client: Arc<Provider<Http>>,
) -> Result<(bool, H160)> {
    let is_option = false;
    let underlying = H160::zero();

    let possible_option_address = address.parse::<Address>()?;
    let o_token = oTOKEN::new(possible_option_address, client.clone());

    let underlying_address = o_token.underlying_token().call().await;

    match underlying_address {
        Ok(underlying_address) => {
            if underlying_address == H160::zero() {
                return Ok((is_option, underlying));
            }
            Ok((true, underlying_address))
        }
        Err(_) => Ok((is_option, underlying)),
    }
}

///
/// Check option liquid discount. Note, returns asian discount (reversed).
///
pub async fn check_option_discount(address: &str, client: Arc<Provider<Http>>) -> Result<f64> {
    let option_address = address.parse::<Address>()?;
    let o_token = oTOKEN::new(option_address, client.clone());

    let discount = o_token.discount().call().await?;

    Ok(discount.as_u64() as f64)
}

///
/// Check option ve discount. Note, returns asian discount (reversed).
///
pub async fn check_option_ve_discount(address: &str, client: Arc<Provider<Http>>) -> Result<f64> {
    let option_address = address.parse::<Address>()?;
    let o_token = oTOKEN::new(option_address, client.clone());

    let discount = o_token.ve_discount().call().await?;

    Ok(discount.as_u64() as f64)
}

abigen!(
    WbltPriceFeed,
    r#"[
        getLivePrice() public view returns (uint256)
    ]"#,
);

///
/// Get wBLT price using wBLT price feed contract. Returns zero if Contract Logic error.
///
pub async fn get_wblt_price(client: Arc<Provider<Http>>) -> Result<f64> {
    let price_feed_address = "0x03dCf91e8e5e07B563d1f2E115B2377f71fE50Aa"
        .parse::<Address>()
        .expect("Address is set by hand");
    let price_feed = WbltPriceFeed::new(price_feed_address, client.clone());
    let price: U256 = price_feed.get_live_price().call().await?;
    let price = format_units(price, 18)?.parse::<f64>()?;
    Ok(price)
}

///
/// Write asset to DB.
///
async fn write_asset(
    conn: &Arc<DatabaseConnection>,
    asset_address: String,
    asset: ActiveAsset,
) -> Result<(), StatusCode> {
    match Assets::insert(asset)
        .on_conflict(
            sea_query::OnConflict::columns([AssetColumn::Address, AssetColumn::ChainId])
                .update_column(AssetColumn::Price)
                .to_owned(),
        )
        .exec(conn.as_ref())
        .await
        .map_err(internal_error)
    {
        Ok(_) => {}
        Err(e) => {
            error!("Error writing asset {} to DB: {:?}", asset_address, e);
            return Err(e);
        }
    }
    info!("Asset {} DB write successful", asset_address);
    Ok(())
}
