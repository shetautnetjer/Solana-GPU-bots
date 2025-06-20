use anyhow::{Result, anyhow};
use std::path::PathBuf;
use tokio::time::{sleep, Duration};
use log::{info, error, warn};
use crate::gpu::GpuScorer;
use crate::types::{PoolUpdate, QuoteResponse};

pub async fn run_monitor(pair: String, interval: u64, window: u64) -> Result<()> {
    info!("Running monitor for pair {} with {}ms interval and {}ms window", pair, interval, window);
    
    let whitelist = crate::load_mint_whitelist()?;
    if whitelist.is_empty() {
        warn!("Whitelist is empty, monitoring will proceed but may include untrusted pools.");
    }
    
    let parts: Vec<&str> = pair.split('/').collect();
    if parts.len() != 2 {
        return Err(anyhow!("Invalid pair format. Expected BASE/QUOTE, e.g., SOL/USDC"));
    }
    let (base_mint, quote_mint) = (parts[0], parts[1]);

    if !whitelist.is_empty() && (!whitelist.contains(base_mint) || !whitelist.contains(quote_mint)) {
        return Err(anyhow!(
            "Pair {}/{} is not in the whitelist. Please add both mints to continue.",
            base_mint, quote_mint
        ));
    }
    info!("Pair {}/{} is in whitelist. Proceeding.", base_mint, quote_mint);

    let pools = crate::jupiter::get_pools_for_pair(base_mint, quote_mint).await?;
    info!("Starting to monitor {} pools for pair {}/{}", pools.len(), base_mint, quote_mint);

    // TODO: Implement WebSocket subscription and data processing for these pools
    for pool in pools {
        info!("-> Subscribing to updates for pool: {}", pool);
    }
    
    println!("\nMonitor is running. Press Ctrl+C to stop.");
    tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;

    Ok(())
}

pub async fn run_quote(
    pair: String,
    amount: u64,
    slippage_bps: u16,
) -> Result<()> {
    info!("Getting quote for pair: {}", pair);
    info!("Amount: {}", amount);
    info!("Slippage: {} bps", slippage_bps);
    
    // Parse the pair
    let parts: Vec<&str> = pair.split('/').collect();
    if parts.len() != 2 {
        return Err(anyhow!("Invalid pair format. Expected BASE/QUOTE"));
    }
    let input_mint = parts[0];
    let output_mint = parts[1];
    
    // Build Jupiter API URL
    let jupiter_api_url = format!(
        "https://lite-api.jup.ag/swap/v1/quote?inputMint={}&outputMint={}&amount={}&slippageBps={}",
        input_mint, output_mint, amount, slippage_bps
    );
    
    info!("Fetching quote from Jupiter API...");
    
    // Make the request
    let client = reqwest::Client::new();
    let response = client
        .get(&jupiter_api_url)
        .send()
        .await?;
    
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(anyhow!("Jupiter API error {}: {}", status, error_text));
    }
    
    let quote: serde_json::Value = response.json().await?;
    
    // Pretty print the quote
    println!("\n📊 Quote Result:");
    println!("{}", serde_json::to_string_pretty(&quote)?);
    
    // Extract key information
    if let Some(routes) = quote.get("data") {
        if let Some(route_plan) = routes.get("routePlan") {
            if let Some(swap_info) = route_plan.as_array().and_then(|arr| arr.first()) {
                if let Some(out_amount) = swap_info.get("swapInfo").and_then(|si| si.get("outAmount")) {
                    println!("\n💰 Expected output: {} (raw amount)", out_amount);
                }
            }
        }
    }
    
    Ok(())
}
