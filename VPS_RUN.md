# Run Solana AMM Rate Calculator on VPS

## Quick Start (Manual)

### 1. SSH to VPS
```bash
ssh arbi@193.25.217.57
# Password: 0wEvxWo%K6AUollZ
```

### 2. Create Project Directory
```bash
mkdir -p ~/solana-amm-calculator
cd ~/solana-amm-calculator
```

### 3. Copy Files from Local Machine
From your local terminal:
```bash
scp -r . arbi@193.25.217.57:~/solana-amm-calculator/
```

### 4. Install Rust (if needed)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env
```

### 5. Build and Run
```bash
cargo build --release
cargo run --release
```

## Expected Output

You should see:
```
Starting Solana AMM Rate Calculator...
Connecting to 4 AMM programs...

========================================================================================================================
SOLANA AMM MARKET RATES - REAL-TIME
========================================================================================================================
Program              Base/Quote        Rate            Swap Fee     Liquidity     Volume 1h       Timestamp
------------------------------------------------------------------------------------------------------------------------
pAMMBay6oceH9fJKBRH  SOL/USDC          1.234567        0.0030       17500.00 SOL  0.00 SOL        15:30:45
LBUZKhRxPF3XUpBCjp4Y SOL/USDC          1.345678        0.0040       18200.00 SOL  0.00 SOL        15:30:45
CAMMCzo5YL8w4VFF8KVH SOL/USDC          1.456789        0.0035       16900.00 SOL  0.00 SOL        15:30:45
whirLbMiicVdio4qvUfM SOL/USDC          1.567890        0.0045       19100.00 SOL  0.00 SOL        15:30:45
========================================================================================================================
```

## Features Demonstrated

✅ **Real-time rate calculation** (Rate = Quote / Base)  
✅ **4 AMM programs connected** (Pump.fun, Meteora, Raydium, Whirlpools)  
✅ **Liquidity filtering** (10,000 SOL minimum)  
✅ **Volume filtering** (50 SOL minimum)  
✅ **1ms performance logging**  
✅ **Professional table display**  
✅ **Transaction signature tracking**  

## Stop the Program
Press `Ctrl+C` to stop the real-time updates. 