// -------- src/jupiter.rs --------------------------------------
use reqwest::{Client, StatusCode};
use serde_json::Value;

#[derive(Debug)]
pub enum JupiterError {
    Http(reqwest::Error),
    Status(StatusCode),
}

impl From<reqwest::Error> for JupiterError {
    fn from(err: reqwest::Error) -> Self {
        JupiterError::Http(err)
    }
}

const DEFAULT_URL: &str = "https://quote-api.jup.ag/v6/indexed-route-map";

pub async fn fetch_pool_token_accounts(pair: &str) -> Result<Vec<String>, JupiterError> {
    fetch_pool_token_accounts_from_url(DEFAULT_URL, pair).await
}

pub async fn fetch_pool_token_accounts_from_url(url: &str, pair: &str) -> Result<Vec<String>, JupiterError> {
    let client = Client::new();
    let res = client.get(url).send().await?;
    if !res.status().is_success() {
        return Err(JupiterError::Status(res.status()));
    }
    let res: Value = res.json().await?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::MockServer;

    #[tokio::test]
    async fn non_200_returns_error() {
        let server = MockServer::start_async().await;
        server
            .mock_async(|when, then| {
                when.method("GET");
                then.status(500);
            })
            .await;

        let res = fetch_pool_token_accounts_from_url(&server.url("/"), "A/B").await;
        match res {
            Err(JupiterError::Status(code)) => assert_eq!(code.as_u16(), 500),
            _ => panic!("unexpected result: {:?}", res),
        }
    }
}
