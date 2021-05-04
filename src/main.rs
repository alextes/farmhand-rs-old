mod base;
mod id;
mod price;
mod price_change;

use async_std::sync::Mutex;
use base::Base;
use lru::LruCache;
use serde::Deserialize;
use std::sync::Arc;
use tide::prelude::json;
use tide::{Request, Response, StatusCode};

pub type HistoricPriceCache = Arc<Mutex<LruCache<String, f64>>>;

#[derive(Clone)]
pub struct State {
    cache: HistoricPriceCache,
}

async fn handle_get_price(req: Request<State>) -> tide::Result {
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

    let prices = price::get_multi_price(id).await?;

    Ok(Response::builder(StatusCode::Ok)
        .body(json!(prices))
        .build())
}

async fn handle_get_price_change(mut req: Request<State>) -> tide::Result {
    let Body { base, days_ago } = req.body_json().await?;
    let symbol = req.param("symbol").unwrap();

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Body {
        base: Base,
        days_ago: i32,
    }

    let id_map = id::get_coingecko_id_map().await?;

    // TODO: pick the token with the highest market cap
    let m_id = id_map.get(symbol.clone()).and_then(|ids| ids.first());
    let id = match m_id {
        Some(id) => id,
        None => {
            return Ok(Response::builder(StatusCode::NotFound)
                .body(format!("no coingecko symbol found for {}", symbol))
                .build())
        }
    };

    let state = req.state();
    let historic_price_cache = Arc::clone(&state.cache);
    let historic_prices =
        price_change::get_price_change(historic_price_cache, id, &base, &days_ago.clone()).await?;
    Ok(Response::builder(StatusCode::Ok)
        .body(json!(historic_prices))
        .build())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    // let mut app = tide::new();

    let cache = Arc::new(Mutex::new(LruCache::new(100000)));
    let mut app = tide::with_state(State { cache });

    app.at("/coin/:symbol/price").get(handle_get_price);
    app.at("/coin/:symbol/price-change/")
        .post(handle_get_price_change);

    if cfg!(debug_assertions) {
        app.listen("127.0.0.1:8080").await?;
    } else {
        // Docker needs binding to 0.0.0.0
        app.listen("0.0.0.0:8080").await?;
    }

    Ok(())
}
