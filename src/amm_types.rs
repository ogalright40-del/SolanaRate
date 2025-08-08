use serde::{Deserialize, Serialize};

// AMM Program IDs
pub const PUMP_FUN_AMM: &str = "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA";
pub const METEORA_DLMM: &str = "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo";
pub const RAYDIUM_CL: &str = "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK";
pub const WHIRLPOOLS: &str = "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPair {
    pub base_token: String,
    pub quote_token: String,
    pub base_mint: String,
    pub quote_mint: String,
    pub base_decimals: u8,
    pub quote_decimals: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolLiquidity {
    pub base_liquidity: f64,
    pub quote_liquidity: f64,
    pub total_liquidity_usd: f64,
    pub volume_24h: f64,
    pub volume_1h: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketRate {
    pub program_id: String,
    pub pool_address: String,
    pub token_pair: TokenPair,
    pub rate: f64,  // Quote / Base
    pub swap_fee: f64,
    pub liquidity: PoolLiquidity,
    pub timestamp: i64,
    pub transaction_signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceUpdate {
    pub market_rate: MarketRate,
    pub price_change_24h: f64,
    pub price_change_1h: f64,
    pub meets_liquidity_filter: bool,
    pub meets_volume_filter: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterConfig {
    pub min_liquidity_sol: f64,  // 10,000 SOL
    pub min_volume_sol: f64,     // 50 SOL
    pub volume_timeframe_ms: i64, // 1,000ms
}

impl Default for FilterConfig {
    fn default() -> Self {
        Self {
            min_liquidity_sol: 10_000.0,
            min_volume_sol: 50.0,
            volume_timeframe_ms: 1_000,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AmmProgram {
    pub id: String,
    pub name: String,
    pub endpoint: String,
}

pub fn get_amm_programs() -> Vec<AmmProgram> {
    vec![
        AmmProgram {
            id: PUMP_FUN_AMM.to_string(),
            name: "Pump.fun AMM".to_string(),
            endpoint: "http://ams2.corvus-labs.io:10101".to_string(),
        },
        AmmProgram {
            id: METEORA_DLMM.to_string(),
            name: "Meteora DLMM".to_string(),
            endpoint: "http://86.105.224.13:10101".to_string(),
        },
        AmmProgram {
            id: RAYDIUM_CL.to_string(),
            name: "Raydium CL".to_string(),
            endpoint: "http://ams2.corvus-labs.io:10101".to_string(),
        },
        AmmProgram {
            id: WHIRLPOOLS.to_string(),
            name: "Whirlpools".to_string(),
            endpoint: "http://86.105.224.13:10101".to_string(),
        },
    ]
} 