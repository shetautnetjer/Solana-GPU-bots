// main.rs
mod jupiter;
mod wss;
mod logger;

use clap::Parser;
use crossbeam::channel;
use rayon::prelude::*;
use std::sync::Arc;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    pair: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Fetch token accounts from Jupiter
    let token_accounts = jupiter::fetch_pool_token_accounts(&args.pair).await.unwrap();
    println!("Fetched {} token accounts", token_accounts.len());

    // Set up logging channel
    let (tx, rx) = channel::bounded(10000);
    std::thread::spawn(move || logger::run_logger(rx));

    // Spawn WS listeners in parallel
    let tx_arc = Arc::new(tx);
    token_accounts.par_iter().for_each(|account| {
        let tx_clone = tx_arc.clone();
        tokio::spawn(wss::subscribe_to_account(account.clone(), tx_clone));
    });
}
