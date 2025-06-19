// -------- src/main.rs -----------------------------------------
mod jupiter;
mod wss;
mod logger;
pub mod types;

use clap::Parser;
use crossbeam::channel;
use rayon::prelude::*;
use crate::types::types::LogRow;

#[derive(Parser, Debug)]
struct Args {
    /// BASE/QUOTE mint addresses
    #[arg(long)]
    pair: String,

    /// Output CSV file path
    #[arg(long, default_value = "pool_updates.csv")]
    logfile: String,

    /// Only log updates from this owner pubkey
    #[arg(long)]
    filter_owner: Option<String>,
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
    let log_path = args.logfile.clone();
    std::thread::spawn(move || logger::run_logger(rx, &log_path));

    // 3️⃣ Spawn one WS listener per account (parallel)
    token_accounts.par_iter().for_each(|account| {
        let tx_clone = tx.clone();
        let acc_clone = account.clone();
        let owner_filter = args.filter_owner.clone();
        tokio::spawn(async move {
            wss::subscribe_to_account(acc_clone, tx_clone, owner_filter).await;
        });
    });

    // Keep main alive
    tokio::signal::ctrl_c().await.unwrap();
    println!("\n⛔ Ctrl+C received – shutting down.");
}
