// -------- src/main.rs -----------------------------------------
mod jupiter;
mod wss;
mod logger;
pub mod types;

use clap::{Parser, Subcommand};
use crossbeam::channel;
use crate::types::types::LogRow;

#[derive(Parser, Debug)]
#[command(author, version, about = "Solana GPU Trading Bot - CPU-first scaffold")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Monitor pool updates for a token pair
    Monitor {
        /// BASE/QUOTE mint addresses
        #[arg(long)]
        pair: String,

        /// Output CSV file path
        #[arg(long, default_value = "pool_updates.csv")]
        logfile: String,

        /// Only log updates from this owner pubkey
        #[arg(long)]
        filter_owner: Option<String>,
    },
    
    /// Get a quote for swapping tokens
    Quote {
        /// Pair in `BASE/QUOTE` format (mint addresses)
        #[arg(long)]
        pair: String,

        /// Amount of BASE to swap, **in smallest units** (lamports, etc.)
        #[arg(long, default_value_t = 1_000_000u64)]
        amount: u64,

        /// Slippage tolerance, in basis‑points (50 = 0.50 %)
        #[arg(long, default_value_t = 50u16)]
        slippage_bps: u16,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Monitor { pair, logfile, filter_owner } => {
            run_monitor(pair, logfile, filter_owner).await;
        }
        Commands::Quote { pair, amount, slippage_bps } => {
            run_quote(pair, amount, slippage_bps).await;
        }
    }
}

async fn run_monitor(pair: String, logfile: String, filter_owner: Option<String>) {
    println!("[🔍] Starting pool monitor for pair: {}", pair);
    
    // 1️⃣ Fetch token accounts via Jupiter
    let token_accounts = match jupiter::fetch_pool_token_accounts(&pair).await {
        Ok(accounts) => {
            println!("[✅] Fetched {} token accounts", accounts.len());
            accounts
        }
        Err(e) => {
            eprintln!("[❌] Failed to fetch token accounts: {}", e);
            std::process::exit(1);
        }
    };

    if token_accounts.is_empty() {
        eprintln!("[⚠️] No token accounts found for pair: {}", pair);
        std::process::exit(1);
    }

    // 2️⃣ Channel for LogRow structs
    let (tx, rx) = channel::bounded::<LogRow>(10_000);
    let log_path = logfile.clone();
    std::thread::spawn(move || logger::run_logger(rx, &log_path));

    // 3️⃣ Spawn one WS listener per account (parallel)
    let mut handles = Vec::new();
    for account in token_accounts {
        let tx_clone = tx.clone();
        let acc_clone = account.clone();
        let owner_filter = filter_owner.clone();
        let handle = tokio::spawn(async move {
            wss::subscribe_to_account(acc_clone, tx_clone, owner_filter).await;
        });
        handles.push(handle);
    }

    println!("[🚀] Monitoring {} accounts. Press Ctrl+C to stop.", handles.len());

    // Keep main alive
    tokio::signal::ctrl_c().await.unwrap();
    println!("\n⛔ Ctrl+C received – shutting down.");
}

async fn run_quote(pair: String, amount: u64, slippage_bps: u16) {
    // Split the pair into two mint addresses
    let (base, quote) = match pair.split_once('/') {
        Some((b, q)) => (b, q),
        None => {
            eprintln!("[❌] --pair must be formatted as BASE/QUOTE mint addresses");
            std::process::exit(1);
        }
    };

    println!(
        "[💡] Quoting {} → {} | amount = {} | slippage = {} bps",
        base, quote, amount, slippage_bps
    );

    match jupiter::quote_exact_in(base, quote, amount, slippage_bps).await {
        Ok(json) => {
            println!("[🚀] Jupiter route OK\n{:#}", json);
        }
        Err(e) => {
            eprintln!("[❌] Jupiter error: {}", e);
            std::process::exit(1);
        }
    }
}
