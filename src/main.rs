use async_std::task;
use cached::proc_macro::cached;
use serde::{Deserialize, Serialize};
use tide::{Request, Response, StatusCode};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum BaseCurrency {
    USD,
}

fn base_currency_to_string(base: BaseCurrency) -> String {
    match base {
        BaseCurrency::USD => "usd".to_string(),
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct Coin {
    symbol: String,
    current_price: f64,
}

async fn fetch_coins_page(base: BaseCurrency, page: u8) -> surf::Result<Vec<Coin>> {
    let uri = format!(
        "https://api.coingecko.com/api/v3/coins/markets?vs_currency={}&order=market_cap_desc&per_page=250&page={}&sparkline=false",
        base_currency_to_string(base),
        page
    );
    surf::get(uri).recv_json().await
}

#[cached(time = 600, result = true)]
async fn fetch_coins(base: BaseCurrency) -> surf::Result<Vec<Coin>> {
    let mut handles = vec![];
    for page_nr in 1..5 {
        let handle = task::spawn(async move { fetch_coins_page(base, page_nr).await });
        handles.push(handle)
    }

    let mut coins = vec![];
    for handle in handles {
        for coin in handle.await? {
            coins.push(coin);
        }
    }

    Ok(coins)
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let mut app = tide::new();

    app.at("/coin/:symbol/price")
        .get(|req: Request<()>| async move {
            let symbol = req.param("symbol").unwrap();

            let on_unknown_error = |e| {
                println!("{}", e);
                Response::builder(StatusCode::InternalServerError)
            };

            Ok(fetch_coins(BaseCurrency::USD)
                .await
                .map_or_else(on_unknown_error, |coins| {
                    coins
                        .iter()
                        .find(|&coin| coin.symbol == symbol)
                        .map_or_else(
                            || {
                                Response::builder(StatusCode::NotFound)
                                    .body(format!("coin {} not found", symbol))
                            },
                            |coin| {
                                Response::builder(StatusCode::Ok)
                                    .body(coin.current_price.to_string())
                            },
                        )
                }))
        });

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
