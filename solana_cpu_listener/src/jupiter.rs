// -------- src/jupiter.rs --------------------------------------
use reqwest::{Client, StatusCode};
use serde_json::Value;
use thiserror::Error;
use std::collections::HashSet;

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

/// Fetch pool token accounts for a given pair by requesting a quote and extracting pool accounts
pub async fn fetch_pool_token_accounts(pair: &str) -> Result<Vec<String>, JupiterError> {
    // Split the pair into two mint addresses
    let (input_mint, output_mint) = pair.split_once('/')
        .ok_or_else(|| JupiterError::InvalidPair(pair.to_string()))?;
    
    // Use a small amount for discovery (1 SOL = 1_000_000_000 lamports)
    let amount = 1_000_000_000u64;
    let slippage_bps = 50u16;
    
    let url = format!(
        "https://lite-api.jup.ag/swap/v1/quote?inputMint={}&outputMint={}&amount={}&slippageBps={}&restrictIntermediateTokens=true",
        input_mint, output_mint, amount, slippage_bps
    );
    
    let res = Client::new()
        .get(&url)
        .header("accept", "application/json")
        .send()
        .await?;
    
    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        return Err(JupiterError::Status { status, body });
    }
    
    let json: Value = res.json().await?;
    
    // Extract pool accounts from routePlan
    let mut accounts = HashSet::new();
    if let Some(routes) = json.get("routePlan").and_then(|v| v.as_array()) {
        for route in routes {
            if let Some(swap_info) = route.get("swapInfo") {
                if let Some(amm_key) = swap_info.get("ammKey").and_then(|v| v.as_str()) {
                    accounts.insert(amm_key.to_string());
                }
            }
        }
    }
    
    if accounts.is_empty() {
        return Err(JupiterError::NoRoute);
    }
    
    Ok(accounts.into_iter().collect())
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
        "https://lite-api.jup.ag/swap/v1/quote?inputMint={}&outputMint={}&amount={}&slippageBps={}&restrictIntermediateTokens=true",
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
    // The old test for fetch_pool_token_accounts_from_url is no longer relevant and has been removed.
}
