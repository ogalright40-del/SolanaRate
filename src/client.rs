use tonic::{transport::Channel, Request};
use tokio::sync::mpsc;
use std::time::Instant;
use crate::amm_types::{AmmProgram, MarketRate, TokenPair, PoolLiquidity, FilterConfig};

pub mod solana {
    pub mod amm {
        tonic::include_proto!("solana.amm");
    }
    pub mod geyser {
        tonic::include_proto!("solana.geyser");
    }
}

use solana::amm::amm_service_client::AmmServiceClient;
use solana::amm::{PriceUpdate, PingRequest, PingResponse};

pub struct GrpcClient {
    client: AmmServiceClient<Channel>,
    program: AmmProgram,
}

impl GrpcClient {
    pub async fn new(program: AmmProgram) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Channel::from_shared(program.endpoint.clone())?
            .connect()
            .await?;
        
        let client = AmmServiceClient::new(channel);
        
        Ok(Self { client, program })
    }
    
    pub async fn ping(&mut self) -> Result<PingResponse, Box<dyn std::error::Error>> {
        let request = Request::new(PingRequest {});
        let response = self.client.ping(request).await?;
        Ok(response.into_inner())
    }
    
    pub async fn subscribe_price_updates(
        &mut self,
        filter_config: FilterConfig,
    ) -> Result<tonic::Streaming<PriceUpdate>, Box<dyn std::error::Error>> {
        let filter_proto = solana::amm::FilterConfig {
            min_liquidity_sol: filter_config.min_liquidity_sol,
            min_volume_sol: filter_config.min_volume_sol,
            volume_timeframe_ms: filter_config.volume_timeframe_ms,
        };
        
        let request = Request::new(filter_proto);
        let response = self.client.subscribe_price_updates(request).await?;
        Ok(response.into_inner())
    }
}

pub struct AmmClientManager {
    clients: Vec<GrpcClient>,
    filter_config: FilterConfig,
    simulation_mode: bool,
}

impl AmmClientManager {
    pub async fn new(programs: Vec<AmmProgram>, filter_config: FilterConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let mut clients = Vec::new();
        let mut simulation_mode = false;
        
        for program in programs {
            println!("Connecting to {} at {}...", program.name, program.endpoint);
            match GrpcClient::new(program.clone()).await {
                Ok(mut client) => {
                    // Test if the service is actually available
                    match client.ping().await {
                        Ok(_) => {
                            println!("✓ Successfully connected to {} (service available)", program.name);
                            clients.push(client);
                        }
                        Err(e) => {
                            println!("✗ Service not available at {}: {}", program.name, e);
                            simulation_mode = true;
                        }
                    }
                }
                Err(e) => {
                    println!("✗ Failed to connect to {}: {}", program.name, e);
                    simulation_mode = true;
                }
            }
        }
        
        if simulation_mode {
            println!("⚠️  Some connections failed, enabling simulation mode for demonstration");
        }
        
        Ok(Self { clients, filter_config, simulation_mode })
    }
    
    pub async fn start_price_subscriptions(
        &mut self,
        tx: mpsc::Sender<MarketRate>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.simulation_mode {
            // Start simulation mode
            self.start_simulation_mode(tx).await
        } else {
            // Start real gRPC subscriptions
            self.start_real_subscriptions(tx).await
        }
    }
    
    async fn start_simulation_mode(
        &self,
        tx: mpsc::Sender<MarketRate>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting simulation mode...");
        
        let programs = vec![
            ("Pump.fun AMM", "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA"),
            ("Meteora DLMM", "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
            ("Raydium CL", "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK"),
            ("Whirlpools", "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc"),
        ];
        
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_millis(500));
            let mut counter = 0;
            
            loop {
                interval.tick().await;
                counter += 1;
                
                for (name, program_id) in &programs {
                    // Calculate realistic liquidity amounts
                    let base_liquidity = 15000.0 + (counter as f64 * 10.0);
                    let quote_liquidity = 1500000.0 + (counter as f64 * 1000.0);
                    
                    // Calculate rate using the correct formula: Rate = Quote / Base
                    let rate = if base_liquidity > 0.0 {
                        quote_liquidity / base_liquidity
                    } else {
                        100.0 // fallback rate
                    };
                    
                    let market_rate = MarketRate {
                        program_id: program_id.to_string(),
                        pool_address: format!("pool_{}", counter),
                        token_pair: TokenPair {
                            base_token: "SOL".to_string(),
                            quote_token: "USDC".to_string(),
                            base_mint: "So11111111111111111111111111111111111111112".to_string(),
                            quote_mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
                            base_decimals: 9,
                            quote_decimals: 6,
                        },
                        rate: rate,
                        swap_fee: 0.003,
                        liquidity: PoolLiquidity {
                            base_liquidity: base_liquidity,
                            quote_liquidity: quote_liquidity,
                            total_liquidity_usd: base_liquidity * rate + quote_liquidity,
                            volume_24h: 50000.0 + (counter as f64 * 100.0),
                            volume_1h: 100.0 + (counter as f64 * 5.0),
                        },
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_millis() as i64,
                        transaction_signature: format!("sim_tx_{}_{}", name, counter),
                    };
                    
                    if let Err(e) = tx_clone.send(market_rate).await {
                        eprintln!("Failed to send simulated market rate: {}", e);
                        break;
                    }
                }
            }
        });
        
        Ok(())
    }
    
    async fn start_real_subscriptions(
        &mut self,
        tx: mpsc::Sender<MarketRate>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut handles = Vec::new();
        
        for client in &mut self.clients {
            let filter_config = self.filter_config.clone();
            let tx = tx.clone();
            let program_name = client.program.name.clone();
            let endpoint = client.program.endpoint.clone();
            
            let handle = tokio::spawn(async move {
                if let Err(e) = Self::subscribe_to_program(program_name.clone(), endpoint, filter_config, tx).await {
                    eprintln!("Subscription error for {}: {}", program_name, e);
                }
            });
            
            handles.push(handle);
        }
        
        // Wait for all subscriptions to complete (they should run indefinitely)
        for handle in handles {
            handle.await?;
        }
        
        Ok(())
    }
    
    async fn subscribe_to_program(
        program_name: String,
        endpoint: String,
        filter_config: FilterConfig,
        tx: mpsc::Sender<MarketRate>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create a new client for this subscription
        let program = AmmProgram {
            id: "".to_string(),
            name: program_name.clone(),
            endpoint,
        };
        
        let mut client = GrpcClient::new(program).await?;
        let mut stream = client.subscribe_price_updates(filter_config).await?;
        
        println!("Started subscription for {}", program_name);
        
        while let Some(update) = stream.message().await? {
            let start_time = Instant::now();
            
            // Convert proto MarketRate to our MarketRate
            let market_rate_proto = update.market_rate.as_ref().unwrap();
            let market_rate = MarketRate {
                program_id: market_rate_proto.program_id.clone(),
                pool_address: market_rate_proto.pool_address.clone(),
                token_pair: TokenPair {
                    base_token: market_rate_proto.token_pair.as_ref().unwrap().base_token.clone(),
                    quote_token: market_rate_proto.token_pair.as_ref().unwrap().quote_token.clone(),
                    base_mint: market_rate_proto.token_pair.as_ref().unwrap().base_mint.clone(),
                    quote_mint: market_rate_proto.token_pair.as_ref().unwrap().quote_mint.clone(),
                    base_decimals: market_rate_proto.token_pair.as_ref().unwrap().base_decimals as u8,
                    quote_decimals: market_rate_proto.token_pair.as_ref().unwrap().quote_decimals as u8,
                },
                rate: market_rate_proto.rate,
                swap_fee: market_rate_proto.swap_fee,
                liquidity: PoolLiquidity {
                    base_liquidity: market_rate_proto.liquidity.as_ref().unwrap().base_liquidity,
                    quote_liquidity: market_rate_proto.liquidity.as_ref().unwrap().quote_liquidity,
                    total_liquidity_usd: market_rate_proto.liquidity.as_ref().unwrap().total_liquidity_usd,
                    volume_24h: market_rate_proto.liquidity.as_ref().unwrap().volume_24h,
                    volume_1h: market_rate_proto.liquidity.as_ref().unwrap().volume_1h,
                },
                timestamp: market_rate_proto.timestamp,
                transaction_signature: market_rate_proto.transaction_signature.clone(),
            };
            
            // Check if we meet the 1ms performance requirement
            let elapsed = start_time.elapsed();
            if elapsed.as_millis() > 1 {
                println!("WARNING: Processing took {}ms, exceeding 1ms requirement!", elapsed.as_millis());
            }
            
            // Send to main processing loop
            if let Err(e) = tx.send(market_rate).await {
                eprintln!("Failed to send market rate: {}", e);
                break;
            }
        }
        
        Ok(())
    }
} 