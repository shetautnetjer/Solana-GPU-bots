// jupiter.rs
use reqwest::Client;
use serde_json::Value;

pub async fn fetch_pool_token_accounts(pair: &str) -> Result<Vec<String>, reqwest::Error> {
    let client = Client::new();
    let url = "https://public.jupiterapi.com/v1/indexed-route-map";

    let res = client.get(url).send().await?.json::<Value>().await?;

    let tokens: Vec<&str> = pair.split('/').collect();
    let base = tokens.get(0).unwrap_or(&"");
    let quote = tokens.get(1).unwrap_or(&"");

    let map = &res["indexedRouteMap"];
    let pool_keys = map.get(base).and_then(|m| m.get(quote));

    if let Some(Value::Array(pools)) = pool_keys {
        let accounts: Vec<String> = pools
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
        Ok(accounts)
    } else {
        Ok(vec![])
    }
}