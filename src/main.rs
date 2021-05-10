mod base;
mod id;
mod price;
mod price_change;
mod request;

use async_std::sync::{Arc, Mutex};
use lru::LruCache;
use price_change::HistoricPriceCache;
use tide::log;

#[derive(Clone)]
pub struct ServerState {
    historic_price_cache: HistoricPriceCache,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let historic_price_cache = Arc::new(Mutex::new(LruCache::new(100000)));

    if cfg!(debug_assertions) {
        log::start();
    } else {
        log::with_level(tide::log::LevelFilter::Warn);
    }

    let mut app = tide::with_state(ServerState {
        historic_price_cache,
    });

    app.at("/coin/:symbol/price").get(price::handle_get_price);
    app.at("/coin/:symbol/price-change/")
        .post(price_change::handle_get_price_change);

    if cfg!(debug_assertions) {
        app.listen("127.0.0.1:8080").await?;
    } else {
        // Docker needs binding to 0.0.0.0
        app.listen("0.0.0.0:8080").await?;
    }

    Ok(())
}
