#!/bin/bash

echo "Deploying to VPS and running Solana AMM Rate Calculator..."

VPS_HOST="arbi@193.25.217.57"
VPS_PASSWORD="0wEvxWo%K6AUollZ"

# Copy project to VPS
echo "Copying project files..."
sshpass -p "$VPS_PASSWORD" scp -o StrictHostKeyChecking=no -r . $VPS_HOST:~/solana-amm-calculator/

# Run on VPS
echo "Building and running on VPS..."
sshpass -p "$VPS_PASSWORD" ssh -o StrictHostKeyChecking=no $VPS_HOST << 'EOF'
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
    
    echo "Running Solana AMM Rate Calculator..."
    echo "=========================================="
    cargo run --release
EOF

echo "Deployment complete!" 