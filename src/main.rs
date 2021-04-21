use cached::proc_macro::cached;
use chrono::prelude::*;
use chrono::Duration;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use tide::prelude::json;
use tide::{Request, Response, StatusCode};

#[derive(Clone, Deserialize, Serialize, Debug)]
struct Coin {
    symbol: String,
    current_price: f64,
}

type CoinGeckoIdMap = HashMap<String, String>;

#[derive(Debug, Deserialize)]
struct CoinId {
    id: String,
    symbol: String,
    name: String,
}

#[cached(time = 3600, result = true)]
async fn fetch_coingecko_id_map() -> Result<CoinGeckoIdMap, surf::Error> {
    let coingecko_id_list: Vec<CoinId> = surf::get("https://api.coingecko.com/api/v3/coins/list")
        .recv_json()
        .await?;

    let mut id_map = HashMap::new();
    for raw_id in coingecko_id_list {
        id_map.insert(raw_id.symbol, raw_id.id);
    }

    Ok(id_map)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SimplePrice {
    usd: f64,
    btc: f64,
    eth: f64,
}

#[cached(time = 600, result = true)]
async fn get_prices_by_id(id: String) -> surf::Result<SimplePrice> {
    let uri = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd%2Cbtc%2Ceth",
        id
    );
    let simple_prices: HashMap<String, SimplePrice> = surf::get(uri).recv_json().await?;

    let simple_price = simple_prices.get(&id).unwrap();
    Ok(simple_price.clone())
}

async fn handle_get_price(req: Request<()>) -> tide::Result {
    let symbol = req.param("symbol").unwrap();

    let id_map = fetch_coingecko_id_map().await?;

    let id = match id_map.get(symbol) {
        Some(id) => id,
        None => {
            return Ok(Response::builder(StatusCode::NotFound)
                .body(format!("no coingecko symbol found for {}", symbol))
                .build())
        }
    };

    let simple_prices = get_prices_by_id(id.into()).await?;

    Ok(Response::builder(StatusCode::Ok)
        .body(json!(simple_prices))
        .build())
}

async fn get_price_change(id: String, days_ago: i64) -> surf::Result<SimplePrice> {
    let utc = Utc::now();
    let date = utc - Duration::days(days_ago);
    let uri = format!(
        "https://api.coingecko.com/api/v3/coins/{}/history?date={}",
        id,
        date.format("%d-%m-%Y")
    );

    let history: Value = surf::get(uri).recv_json().await?;
    let historic_price = SimplePrice {
        usd: history["market_data"]["current_price"]["usd"]
            .as_f64()
            .unwrap(),
        btc: history["market_data"]["current_price"]["btc"]
            .as_f64()
            .unwrap(),
        eth: history["market_data"]["current_price"]["eth"]
            .as_f64()
            .unwrap(),
    };

    let current_price = get_prices_by_id(id.into()).await?;

    let price_change = SimplePrice {
        usd: current_price.usd / historic_price.usd,
        btc: current_price.btc / historic_price.btc,
        eth: current_price.eth / historic_price.eth,
    };

    Ok(price_change)
}

async fn handle_get_price_change(req: Request<()>) -> tide::Result {
    let symbol = req.param("symbol").unwrap();
    let days_ago = req.param("days_ago").unwrap();

    let id_map = fetch_coingecko_id_map().await?;

    let id = match id_map.get(symbol) {
        Some(id) => id,
        None => {
            return Ok(Response::builder(StatusCode::NotFound)
                .body(format!("no coingecko symbol found for {}", symbol))
                .build())
        }
    };

    let historic_prices = get_price_change(id.into(), days_ago.parse()?).await?;
    Ok(Response::builder(StatusCode::Ok)
        .body(json!(historic_prices))
        .build())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let mut app = tide::new();

    app.at("/coin/:symbol/price").get(handle_get_price);
    app.at("/coin/:symbol/price-change/:days_ago")
        .get(handle_get_price_change);

    let env = env::var("ENV").unwrap_or("dev".to_string());
    if env == "dev" {
        app.listen("127.0.0.1:8080").await?;
    } else {
        // Docker needs binding to 0.0.0.0
        app.listen("0.0.0.0:8080").await?;
    }

    Ok(())
}
