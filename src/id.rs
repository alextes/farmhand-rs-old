use cached::proc_macro::cached;
use serde::Deserialize;
use std::collections::HashMap;

type IdMap = HashMap<String, Vec<String>>;

#[derive(Debug, Deserialize)]
struct CoinId {
    id: String,
    symbol: String,
    name: String,
}

#[cached(time = 86400, result = true)]
pub async fn get_coingecko_id_map() -> surf::Result<IdMap> {
    let coingecko_id_list: Vec<CoinId> = surf::get("https://api.coingecko.com/api/v3/coins/list")
        .recv_json()
        .await?;

    type IdMap = HashMap<String, Vec<String>>;
    let mut id_map: IdMap = HashMap::new();

    for raw_id in coingecko_id_list {
        let id_list = id_map.entry(raw_id.symbol).or_insert(Vec::new());
        id_list.push(raw_id.id);
    }

    // Some symbols have multiple IDs, we don't support retrieving the map sorted by highest market
    // cap yet, or by contract, so we hardcode some overwrites that are probably returning the
    // token the caller is actually looking for.
    id_map.insert(String::from("uni"), vec![String::from("uniswap")]);
    id_map.insert(String::from("ftt"), vec![String::from("ftx-token")]);

    Ok(id_map)
}

// TODO: getIdMapSortedByMarketCap
