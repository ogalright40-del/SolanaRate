use crate::amm_types::{MarketRate, PoolLiquidity, TokenPair, FilterConfig};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

pub struct RateCalculator {
    filter_config: FilterConfig,
    performance_log: Vec<(String, Instant)>,
}

impl RateCalculator {
    pub fn new(filter_config: FilterConfig) -> Self {
        Self {
            filter_config,
            performance_log: Vec::new(),
        }
    }

    pub fn calculate_rate(&mut self, base_amount: f64, quote_amount: f64) -> f64 {
        let start_time = Instant::now();
        
        // Rate = Quote / Base (USDC per SOL)
        let rate = if base_amount > 0.0 {
            quote_amount / base_amount
        } else {
            0.0
        };

        // Log performance
        let elapsed = start_time.elapsed();
        self.performance_log.push((
            format!("Rate calculation: {}μs", elapsed.as_micros()),
            start_time,
        ));

        rate
    }

    pub fn apply_filters(&self, market_rate: &MarketRate) -> (bool, bool) {
        let meets_liquidity = market_rate.liquidity.total_liquidity_usd >= self.filter_config.min_liquidity_sol;
        let meets_volume = market_rate.liquidity.volume_1h >= self.filter_config.min_volume_sol;
        
        (meets_liquidity, meets_volume)
    }

    pub fn create_market_rate(
        &mut self,
        program_id: String,
        pool_address: String,
        token_pair: TokenPair,
        base_liquidity: f64,
        quote_liquidity: f64,
        swap_fee: f64,
        transaction_signature: String,
    ) -> MarketRate {
        let start_time = Instant::now();
        
        // Calculate rate
        let rate = self.calculate_rate(base_liquidity, quote_liquidity);
        
        // Create liquidity info
        let liquidity = PoolLiquidity {
            base_liquidity,
            quote_liquidity,
            total_liquidity_usd: base_liquidity + quote_liquidity, // Simplified
            volume_24h: 0.0, // Would come from real data
            volume_1h: 0.0,  // Would come from real data
        };

        // Get timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let market_rate = MarketRate {
            program_id,
            pool_address,
            token_pair,
            rate,
            swap_fee,
            liquidity,
            timestamp,
            transaction_signature,
        };

        // Log performance
        let elapsed = start_time.elapsed();
        self.performance_log.push((
            format!("Market rate creation: {}μs", elapsed.as_micros()),
            start_time,
        ));

        market_rate
    }

    pub fn get_performance_log(&self) -> &Vec<(String, Instant)> {
        &self.performance_log
    }

    pub fn log_transaction_detection(&mut self, transaction_signature: &str) {
        let start_time = Instant::now();
        self.performance_log.push((
            format!("Transaction detected: {}", transaction_signature),
            start_time,
        ));
    }

    pub fn log_rate_output(&mut self, market_rate: &MarketRate) {
        let start_time = Instant::now();
        self.performance_log.push((
            format!("Rate output: {} -> {}", market_rate.token_pair.base_token, market_rate.rate),
            start_time,
        ));
    }

    pub fn check_1ms_performance(&self) -> bool {
        // Check if any operation took more than 1ms
        for (log_entry, start_time) in &self.performance_log {
            let elapsed = start_time.elapsed();
            if elapsed.as_millis() > 1 {
                println!("Performance warning: {} took {}ms", log_entry, elapsed.as_millis());
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_calculation() {
        let config = FilterConfig::default();
        let mut calculator = RateCalculator::new(config);
        
        let rate = calculator.calculate_rate(100.0, 200.0);
        assert_eq!(rate, 2.0);
    }

    #[test]
    fn test_filters() {
        let config = FilterConfig::default();
        let calculator = RateCalculator::new(config);
        
        let token_pair = TokenPair {
            base_token: "SOL".to_string(),
            quote_token: "USDC".to_string(),
            base_mint: "".to_string(),
            quote_mint: "".to_string(),
            base_decimals: 9,
            quote_decimals: 6,
        };

        let liquidity = PoolLiquidity {
            base_liquidity: 5000.0,
            quote_liquidity: 5000.0,
            total_liquidity_usd: 10000.0,
            volume_24h: 100.0,
            volume_1h: 60.0,
        };

        let market_rate = MarketRate {
            program_id: "test".to_string(),
            pool_address: "test".to_string(),
            token_pair,
            rate: 1.0,
            swap_fee: 0.003,
            liquidity,
            timestamp: 0,
            transaction_signature: "test".to_string(),
        };

        let (meets_liquidity, meets_volume) = calculator.apply_filters(&market_rate);
        assert!(meets_liquidity);
        assert!(meets_volume);
    }
} 