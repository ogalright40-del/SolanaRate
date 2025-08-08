#!/bin/bash

echo "🚀 Automated Solana AMM Rate Calculator Deployment"
echo "=================================================="

VPS_HOST="arbi@193.25.217.57"

# Prompt for password
echo -n "Enter VPS password: "
read -s VPS_PASSWORD
echo

echo "📁 Copying project files to VPS..."
scp -o StrictHostKeyChecking=no -r . $VPS_HOST:~/solana-amm-calculator/

echo "🔨 Building and running on VPS..."
ssh -o StrictHostKeyChecking=no $VPS_HOST << EOF
    cd ~/solana-amm-calculator
    
    # Install Rust if needed
    if ! command -v rustc &> /dev/null; then
        echo "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source ~/.cargo/env
    fi
    
    # Build and run
    echo "Building project..."
    source ~/.cargo/env
    cargo build --release
    
    echo ""
    echo "🎯 Running Solana AMM Rate Calculator..."
    echo "=========================================="
    cargo run --release
EOF

echo "✅ Deployment complete!" 