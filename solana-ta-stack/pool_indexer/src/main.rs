mod types;
mod gpu;
mod indexer;
mod pyth;
mod jupiter;

use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;
use std::collections::HashSet;

#[derive(Parser, Debug)]
#[command(author, version, about = "Solana GPU Trading Bot - High Performance Pool Indexer")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize GPU and run diagnostics
    GpuTest,
    
    /// Monitor pools for a token pair with GPU-accelerated scoring
    Monitor {
        /// BASE/QUOTE mint addresses
        #[arg(long)]
        pair: String,
        
        /// Output CSV file path
        #[arg(long, default_value = "pool_updates.csv")]
        logfile: String,
        
        /// Enable GPU acceleration for scoring
        #[arg(long)]
        gpu: bool,
    },
    
    /// Get a quote and analyze with GPU
    Quote {
        /// Pair in BASE/QUOTE format
        #[arg(long)]
        pair: String,
        
        /// Amount of BASE to swap
        #[arg(long, default_value_t = 1_000_000u64)]
        amount: u64,
        
        /// Slippage tolerance in basis points
        #[arg(long, default_value_t = 50u16)]
        slippage_bps: u16,
    },
    
    /// Fetch all supported pools for a token pair from Jupiter
    FetchPools {
        /// BASE/QUOTE mint addresses
        #[arg(long)]
        pair: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::GpuTest => {
            println!("🚀 Running GPU diagnostics...");
            match gpu::gpu_smoke_test() {
                Ok(report) => {
                    println!("✅ GPU Test Passed!");
                    println!("{}", serde_json::to_string_pretty(&report)?);
                },
                Err(e) => {
                    eprintln!("❌ GPU Test Failed: {}", e);
                    std::process::exit(1);
                }
            }
        },
        
        Commands::Monitor { pair, logfile, gpu } => {
            println!("🔍 Starting pool monitor for pair: {}", pair);
            if gpu {
                println!("⚡ GPU acceleration enabled");
            }
            
            // Run the monitor with default interval and window
            indexer::run_monitor(pair, 1000, 5000).await?;
        },
        
        Commands::Quote { pair, amount, slippage_bps } => {
            println!("💡 Getting quote for {}", pair);
            indexer::run_quote(pair, amount, slippage_bps).await?;
        },
        
        Commands::FetchPools { pair } => {
            println!("🔍 Fetching pools for pair: {}", pair);
            let parts: Vec<&str> = pair.split('/').collect();
            if parts.len() != 2 {
                return Err(anyhow!("Invalid pair format. Expected BASE/QUOTE"));
            }
            let base_mint = parts[0];
            let quote_mint = parts[1];

            match jupiter::get_pools_for_pair(base_mint, quote_mint).await {
                Ok(pools) => {
                    println!("✅ Found {} pools:", pools.len());
                    for pool_id in pools {
                        println!(" - {}", pool_id);
                    }
                },
                Err(e) => {
                    eprintln!("❌ Error fetching pools: {}", e);
                }
            }
        }
    }
    
    Ok(())
}

fn load_mint_whitelist() -> Result<HashSet<String>> {
    let path = PathBuf::from("whitelist/mint_whitelist.txt");
    if !path.exists() {
        // Try relative to executable if not found
        let exe_path = std::env::current_exe()?
            .parent()
            .unwrap()
            .join("whitelist/mint_whitelist.txt");
        
        if exe_path.exists() {
            let content = std::fs::read_to_string(exe_path)?;
            return Ok(content.lines().map(|s| s.split_whitespace().next().unwrap_or("").to_string()).collect());
        } else {
            println!("⚠️  Mint whitelist not found, proceeding without filter");
            return Ok(HashSet::new());
        }
    }
    let content = std::fs::read_to_string(path)?;
    Ok(content.lines().map(|s| s.split_whitespace().next().unwrap_or("").to_string()).collect())
}
