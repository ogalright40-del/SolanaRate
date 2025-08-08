mod amm_types;
mod rate_calculator;
mod table_ui;

use amm_types::{get_amm_programs, FilterConfig, MarketRate, TokenPair};
use rate_calculator::RateCalculator;
use table_ui::TableUI;
use tokio::time::sleep;
use std::time::Duration;

pub mod solana {
    pub mod amm {
        tonic::include_proto!("solana.amm");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Solana AMM Rate Calculator...");
    
    let filter_config = FilterConfig::default();
    let mut calculator = RateCalculator::new(filter_config);
    let mut table_ui = TableUI::new(20);
    
    let amm_programs = get_amm_programs();
    
    println!("Connecting to {} AMM programs...", amm_programs.len());
    
    // Simulate real-time data for demonstration
    let mut interval = tokio::time::interval(Duration::from_millis(100));
    
    loop {
        interval.tick().await;
        
        for program in &amm_programs {
            // Simulate market rate data
            let market_rate = simulate_market_rate(&mut calculator, program);
            
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
        }
        
        sleep(Duration::from_millis(1000)).await;
    }
}

fn simulate_market_rate(calculator: &mut RateCalculator, program: &amm_types::AmmProgram) -> MarketRate {
    let token_pair = TokenPair {
        base_token: "SOL".to_string(),
        quote_token: "USDC".to_string(),
        base_mint: "So11111111111111111111111111111111111111112".to_string(),
        quote_mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
        base_decimals: 9,
        quote_decimals: 6,
    };
    
    let base_liquidity = 15000.0 + (rand::random::<f64>() * 5000.0);
    let quote_liquidity = 15000.0 + (rand::random::<f64>() * 5000.0);
    let swap_fee = 0.003 + (rand::random::<f64>() * 0.002);
    let transaction_signature = format!("tx_{}", rand::random::<u64>());
    
    calculator.create_market_rate(
        program.id.clone(),
        format!("pool_{}", rand::random::<u64>()),
        token_pair,
        base_liquidity,
        quote_liquidity,
        swap_fee,
        transaction_signature,
    )
}