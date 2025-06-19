// -------- src/main.rs -----------------------------------------
mod jupiter;
mod wss;
mod logger;
mod types;

use clap::Parser;
use crossbeam::channel;
use rayon::prelude::*;
use std::sync::Arc;
use types::LogRow;

#[derive(Parser, Debug)]
struct Args {
    /// BASE/QUOTE mint addresses
    #[arg(long)]
    pair: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // 1️⃣ Fetch token accounts via Jupiter
    let token_accounts = jupiter::fetch_pool_token_accounts(&args.pair)
        .await
        .expect("Failed Jupiter query");
    println!("Fetched {} token accounts", token_accounts.len());

    // 2️⃣ Channel for LogRow structs
    let (tx, rx) = channel::bounded::<LogRow>(10_000);
    std::thread::spawn(move || logger::run_logger(rx));

    // 3️⃣ Spawn one WS listener per account (parallel)
    let tx_arc = Arc::new(tx);
    token_accounts.par_iter().for_each(|account| {
        let tx_clone = tx_arc.clone();
        let acc_clone = account.clone();
        tokio::spawn(async move {
            wss::subscribe_to_account(acc_clone, tx_clone).await;
        });
    });

    // Keep main alive
    tokio::signal::ctrl_c().await.unwrap();
    println!("\n⛔ Ctrl+C received – shutting down.");
}