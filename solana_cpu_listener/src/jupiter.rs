// -------- src/jupiter.rs --------------------------------------
use reqwest::{Client, StatusCode};
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JupiterError {
    #[error("Transport error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Jupiter returned {status}: {body}")]
    Status {
        status: StatusCode,
        body: String,
    },

    #[error("No route available for requested pair/amount")]
    NoRoute,

    #[error("Invalid pair format: {0}")]
    InvalidPair(String),
}

const DEFAULT_URL: &str = "https://quote-api.jup.ag/v6/indexed-route-map";

/// Fetch pool token accounts for a given pair
pub async fn fetch_pool_token_accounts(pair: &str) -> Result<Vec<String>, JupiterError> {
    fetch_pool_token_accounts_from_url(DEFAULT_URL, pair).await
}

/// Fetch pool token accounts from a specific URL
pub async fn fetch_pool_token_accounts_from_url(url: &str, pair: &str) -> Result<Vec<String>, JupiterError> {
    let client = Client::new();
    let res = client.get(url).send().await?;
    
    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        return Err(JupiterError::Status {
            status,
            body,
        });
    }
    
    let res: Value = res.json().await?;

    let mut parts = pair.split('/');
    let base = parts.next().ok_or_else(|| JupiterError::InvalidPair(pair.to_string()))?;
    let quote = parts.next().ok_or_else(|| JupiterError::InvalidPair(pair.to_string()))?;

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

/// Request a quote with **ExactIn** semantics.
///
/// * `input_mint`  – BASE mint address
/// * `output_mint` – QUOTE mint address
/// * `amount`      – amount of BASE in *smallest units* (lamports, etc.)
/// * `slippage_bps` – slippage tolerance, basis‑points
pub async fn quote_exact_in(
    input_mint: &str,
    output_mint: &str,
    amount: u64,
    slippage_bps: u16,
) -> Result<Value, JupiterError> {
    let url = format!(
        "https://quote-api.jup.ag/v6/quote?inputMint={}&outputMint={}&amount={}&swapMode=ExactIn&slippageBps={}",
        input_mint, output_mint, amount, slippage_bps
    );

    let res = reqwest::Client::new()
        .get(&url)
        .header("accept", "application/json")
        .send()
        .await?;

    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        return Err(JupiterError::Status {
            status,
            body,
        });
    }

    let json: Value = res.json().await?;

    // Ensure we actually got at least one route
    if json
        .get("routePlan")
        .and_then(|v| v.as_array())
        .map_or(true, |arr| arr.is_empty())
    {
        return Err(JupiterError::NoRoute);
    }

    Ok(json)
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
            Err(JupiterError::Status { status, .. }) => assert_eq!(status.as_u16(), 500),
            _ => panic!("unexpected result: {:?}", res),
        }
    }

    #[tokio::test]
    async fn invalid_pair_format_returns_error() {
        let server = MockServer::start_async().await;
        server
            .mock_async(|when, then| {
                when.method("GET");
                then.status(200).json_body(serde_json::json!({
                    "indexedRouteMap": {}
                }));
            })
            .await;

        let res = fetch_pool_token_accounts_from_url(&server.url("/"), "invalid").await;
        match res {
            Err(JupiterError::InvalidPair(_)) => {},
            _ => panic!("expected InvalidPair error, got: {:?}", res),
        }
    }
}
