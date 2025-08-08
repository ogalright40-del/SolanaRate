use crate::amm_types::MarketRate;
use std::collections::VecDeque;

pub struct TableUI {
    market_rates: VecDeque<MarketRate>,
    max_rows: usize,
}

impl TableUI {
    pub fn new(max_rows: usize) -> Self {
        Self {
            market_rates: VecDeque::new(),
            max_rows,
        }
    }

    pub fn add_market_rate(&mut self, rate: MarketRate) {
        self.market_rates.push_back(rate);
        if self.market_rates.len() > self.max_rows {
            self.market_rates.pop_front();
        }
    }

    pub fn display_table(&self) {
        println!("\n{}", "=".repeat(120));
        println!("SOLANA AMM MARKET RATES - REAL-TIME");
        println!("{}", "=".repeat(120));
        println!("{:<20} {:<15} {:<15} {:<12} {:<12} {:<15} {:<15}", 
                 "Program", "Base/Quote", "Rate", "Swap Fee", "Liquidity", "Volume 1h", "Timestamp");
        println!("{}", "-".repeat(120));

        for rate in &self.market_rates {
            let pair = format!("{}/{}", rate.token_pair.base_token, rate.token_pair.quote_token);
            let liquidity = format!("{:.2} SOL", rate.liquidity.total_liquidity_usd);
            let volume = format!("{:.2} SOL", rate.liquidity.volume_1h);
            let timestamp = chrono::DateTime::from_timestamp_millis(rate.timestamp)
                .unwrap_or_default()
                .format("%H:%M:%S")
                .to_string();

            println!("{:<20} {:<15} {:<15.6} {:<12.4} {:<12} {:<15} {:<15}", 
                     rate.program_id[..20].to_string(), pair, rate.rate, rate.swap_fee, liquidity, volume, timestamp);
        }
        println!("{}", "=".repeat(120));
    }

    pub fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H");
    }
} 