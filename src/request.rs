use async_std::future;
use serde::de::DeserializeOwned;
use surf::StatusCode;
use std::time::Duration;

pub async fn get_json<A: DeserializeOwned + Clone>(
    duration: Duration,
    uri: &String,
) -> surf::Result<A> {
    let request_f = surf::get(uri).recv_json();
    future::timeout(duration, request_f).await.map_or_else(
        |error| Err(surf::Error::new(StatusCode::RequestTimeout, error)),
        |result: surf::Result<A>| result.map(|result| result.to_owned()),
    )
}
