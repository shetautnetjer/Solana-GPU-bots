use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolUpdate {
    pub pool_address: String,
    pub token_a_reserves: u64,
    pub token_b_reserves: u64,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteResponse {
    pub in_amount: u64,
    pub out_amount: u64,
    pub fee_amount: u64,
    pub fee_mint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricUpdate {
    pub timestamp: i64,
    pub pool_address: String,
    pub price: f64, // From Pyth
    pub reserve_a: u64, // From WebSocket
    pub reserve_b: u64, // From WebSocket
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketInfo {
    pub id: String,
    pub label: String,
    pub input_mint: String,
    pub output_mint: String,
    pub not_enough_liquidity: bool,
    pub in_amount: u64,
    pub out_amount: u64,
    pub min_in_amount: Option<u64>,
    pub min_out_amount: Option<u64>,
    pub price_impact_pct: Option<f64>,
    pub lp_fee: Option<LpFee>,
    pub platform_fee: Option<PlatformFee>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LpFee {
    pub amount: u64,
    pub mint: String,
    pub pct: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformFee {
    pub amount: u64,
    pub mint: String,
    pub pct: Option<f64>,
}
