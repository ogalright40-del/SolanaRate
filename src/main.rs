mod amm_types;
mod rate_calculator;
mod table_ui;
mod client;

use amm_types::{get_amm_programs, FilterConfig, MarketRate};
use rate_calculator::RateCalculator;
use table_ui::TableUI;
use client::AmmClientManager;
use tokio::sync::mpsc;

pub mod solana {
    pub mod amm {
        tonic::include_proto!("solana.amm");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Solana AMM Rate Calculator...");
    
    let filter_config = FilterConfig::default();
    let mut calculator = RateCalculator::new(filter_config.clone());
    let mut table_ui = TableUI::new(20);
    
    let amm_programs = get_amm_programs();
    
    println!("Connecting to {} AMM programs...", amm_programs.len());
    
    // Create gRPC client manager
    println!("Creating gRPC client manager...");
    let mut client_manager = AmmClientManager::new(amm_programs, filter_config).await?;
    println!("gRPC client manager created successfully!");
    
    // Create channel for receiving market rates
    let (tx, mut rx) = mpsc::channel::<MarketRate>(1000);
    
    // Start price subscriptions in background
    println!("Starting price subscriptions...");
    let subscription_handle = tokio::spawn(async move {
        if let Err(e) = client_manager.start_price_subscriptions(tx).await {
            eprintln!("Subscription error: {}", e);
        }
    });
    println!("Price subscriptions started!");
    
    println!("Starting main processing loop...");
    
    // Main processing loop
    println!("Waiting for market rate data...");
    while let Some(market_rate) = rx.recv().await {
        let start_time = std::time::Instant::now();
        
        // Apply filters
        let (meets_liquidity, meets_volume) = calculator.apply_filters(&market_rate);
        
        if meets_liquidity && meets_volume {
            calculator.log_transaction_detection(&market_rate.transaction_signature);
            calculator.log_rate_output(&market_rate);
            
            table_ui.add_market_rate(market_rate);
            table_ui.display_table();
            
            // Check 1ms performance
            if !calculator.check_1ms_performance() {
                println!("WARNING: Performance requirement not met!");
            }
        }
        
        // Log processing time
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > 1 {
            println!("WARNING: Main loop processing took {}ms!", elapsed.as_millis());
        }
    }
    
    // Wait for subscription to complete (shouldn't happen unless error)
    subscription_handle.await?;
    
    Ok(())
}

