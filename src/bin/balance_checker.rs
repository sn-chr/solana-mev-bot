use clap::Parser;
use solana_mev_bot::utils::balance_checker;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// RPC URL for Solana network
    #[clap(short, long, default_value = "https://api.mainnet-beta.solana.com")]
    rpc_url: String,
    
    /// Wallet private key (base58 string or file path)
    #[clap(short, long)]
    private_key: String,
    
    /// Send excess SOL if balance is above 10 SOL
    #[clap(long)]
    send: bool,
    
    /// Minimum balance to keep (in SOL)
    #[clap(long, default_value = "10.0")]
    min_balance: f64,
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_line_number(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let args = Args::parse();
    
    if args.send {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        match balance_checker::check_and_trade(&args.rpc_url, &args.private_key).await {
            Ok(()) => {
            }
            Err(e) => {
                eprintln!("❌ Error: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        match balance_checker::check_balance_only(&args.rpc_url, &args.private_key).await {
            Ok(()) => {
            }
            Err(e) => {
                eprintln!("❌ Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
