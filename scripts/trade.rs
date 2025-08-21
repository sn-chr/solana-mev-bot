use solana_mev_bot::{config::Config, utils::balance_checker};
use std::env;

#[tokio::main]
async fn main() {
    // Load configuration from config.toml
    let config = match Config::load("config.toml") {
        Ok(config) => config,
        Err(e) => {
            eprintln!("❌ Failed to load config.toml: {}", e);
            std::process::exit(1);
        }
    };

    // Check if --send flag is provided
    let args: Vec<String> = env::args().collect();
    let send_mode = args.contains(&"--send".to_string());

    if send_mode {
        match balance_checker::check_and_trade(&config.rpc.url, &config.wallet.private_key).await {
            Ok(()) => {
                println!("✅ Trade execution completed successfully!");
            }
            Err(e) => {
                eprintln!("❌ Error: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        match balance_checker::check_balance_only(&config.rpc.url, &config.wallet.private_key).await {
            Ok(()) => {
                println!("✅ Balance check completed!");
            }
            Err(e) => {
                eprintln!("❌ Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
