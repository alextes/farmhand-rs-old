use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize, Debug)]
struct Coin {
    symbol: String,
    current_price: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MultiPrice {
    usd: f64,
    btc: f64,
    eth: f64,
}

pub async fn get_multi_price(id: &String) -> surf::Result<MultiPrice> {
    let uri = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd%2Cbtc%2Ceth",
        id
    );
    let prices: HashMap<String, MultiPrice> = surf::get(uri).recv_json().await?;

    let price = prices.get(id).unwrap();
    Ok(price.clone())
}
