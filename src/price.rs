use crate::id;
use crate::ServerState;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tide::prelude::*;
use tide::{Request, Response, StatusCode};

#[derive(Clone, Deserialize, Serialize, Debug)]
struct Coin {
    symbol: String,
    current_price: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct MultiPrice {
    usd: f64,
    btc: f64,
    eth: f64,
}

async fn fetch_multi_price(id: &String) -> surf::Result<MultiPrice> {
    let uri = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd%2Cbtc%2Ceth",
        id
    );
    let prices: HashMap<String, MultiPrice> = surf::get(uri).recv_json().await?;

    let price = prices.get(id).unwrap().to_owned();
    Ok(price)
}

pub async fn handle_get_price(req: Request<ServerState>) -> tide::Result {
    let symbol = req.param("symbol").unwrap();

    let id_map = id::get_coingecko_id_map().await?;

    // TODO: pick the token with the highest market cap
    let m_id = id_map.get(symbol).and_then(|ids| ids.first());
    let id = match m_id {
        Some(id) => id,
        None => {
            return Ok(Response::builder(StatusCode::NotFound)
                .body(format!("no coingecko symbol found for {}", symbol))
                .build())
        }
    };

    fetch_multi_price(id).await.map_or_else(
        |err| {
            if err.status() == StatusCode::TooManyRequests {
                Ok(Response::new(StatusCode::TooManyRequests))
            } else {
                Err(err)
            }
        },
        |prices| {
            Ok(Response::builder(StatusCode::Ok)
                .body(json!(prices))
                .build())
        },
    )
}
