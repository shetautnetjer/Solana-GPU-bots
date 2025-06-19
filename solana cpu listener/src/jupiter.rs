// -------- src/jupiter.rs --------------------------------------
use reqwest::Client;
use serde_json::Value;

pub async fn fetch_pool_token_accounts(pair: &str) -> Result<Vec<String>, reqwest::Error> {
    let client = Client::new();
    let url = "https://public.jupiterapi.com/v1/indexed-route-map";
    let res: Value = client.get(url).send().await?.json().await?;

    let mut parts = pair.split('/');
    let base = parts.next().unwrap_or("");
    let quote = parts.next().unwrap_or("");

    let map = &res["indexedRouteMap"];
    let pool_keys = map.get(base).and_then(|m| m.get(quote));

    if let Some(Value::Array(pools)) = pool_keys {
        Ok(pools
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect())
    } else {
        Ok(vec![])
    }
}
