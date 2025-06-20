use anyhow::{Result, anyhow};
use log::info;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MarketInfo {
    id: String,
    label: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Route {
    market_infos: Vec<MarketInfo>,
}

#[derive(Debug, Deserialize)]
struct JupiterQuoteResponse {
    data: Vec<Route>,
}

/// Fetches all available pools for a given token pair from the Jupiter API.
pub async fn get_pools_for_pair(base_mint: &str, quote_mint: &str) -> Result<Vec<String>> {
    info!("Fetching pools for {}/{} from Jupiter", base_mint, quote_mint);

    // Use a nominal amount to get all possible routes/pools
    let url = format!(
        "https://quote-api.jup.ag/v6/quote?inputMint={}&outputMint={}&amount=1000000&slippageBps=50",
        base_mint, quote_mint
    );

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow!(
            "Jupiter API error ({}): {}",
            response.status(),
            error_text
        ));
    }

    let quote_response: JupiterQuoteResponse = response.json().await?;

    let mut pool_ids = HashSet::new();
    for route in quote_response.data {
        for market in route.market_infos {
            info!("Discovered pool: {} ({})", market.id, market.label);
            pool_ids.insert(market.id);
        }
    }

    if pool_ids.is_empty() {
        return Err(anyhow!("No pools found for pair {}/{}", base_mint, quote_mint));
    }

    Ok(pool_ids.into_iter().collect())
} 