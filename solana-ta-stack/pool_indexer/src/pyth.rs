use anyhow::Result;
use pyth_sdk_solana::state::PriceAccount;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use log::info;

pub struct PythClient {
    rpc_client: RpcClient,
}

impl PythClient {
    pub fn new(rpc_url: &str) -> Self {
        Self {
            rpc_client: RpcClient::new(rpc_url.to_string()),
        }
    }

    pub fn get_price(&self, price_feed_id: &str) -> Result<f64> {
        info!("Fetching price from Pyth feed: {}", price_feed_id);
        let account_key = Pubkey::from_str(price_feed_id)?;
        let account_data = self.rpc_client.get_account_data(&account_key)?;
        
        let price_account: &PriceAccount = bytemuck::from_bytes(&account_data);

        let price = price_account.get_price_unchecked();
        
        // Price is returned as i64 with an exponent. Convert to f64.
        let result = (price.price as f64) * 10f64.powi(price.expo);

        info!("Price for {} is {}", price_feed_id, result);
        Ok(result)
    }
}
