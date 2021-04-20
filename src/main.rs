use cached::proc_macro::cached;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

#[derive(Clone, Deserialize, Serialize)]
struct SimplePrice {
    usd: Option<f64>,
    btc: Option<f64>,
    eth: Option<f64>,
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

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let mut app = tide::new();

    app.at("/coin/:symbol/price").get(handle_get_price);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
