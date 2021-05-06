mod base;
mod id;
mod price;
mod price_change;

use async_std::sync::Mutex;
use lru::LruCache;
use std::sync::Arc;
use tide::log;

pub type HistoricPriceCache = Arc<Mutex<LruCache<String, f64>>>;

#[derive(Clone)]
pub struct ServerState {
    cache: HistoricPriceCache,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let cache = Arc::new(Mutex::new(LruCache::new(100000)));

    if cfg!(debug_assertions) {
        log::start();
    } else {
        log::with_level(tide::log::LevelFilter::Warn);
    }

    let mut app = tide::with_state(ServerState { cache });

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
