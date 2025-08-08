pub mod solana {
    pub mod geyser {
        tonic::include_proto!("solana.geyser");
    }
}

use solana::geyser::geyser_client::GeyserClient;
use solana::geyser::PingRequest;
use tonic::transport::Channel;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoints = vec![
        "http://[::1]:50051",
        "http://localhost:50051",
        "http://ams2.corvus-labs.io:10101",
        "http://86.105.224.13:10101",
    ];

    for endpoint in endpoints {
        println!("Testing connection to: {}", endpoint);
        
        let https_url = if endpoint.contains("corvus-labs.io") || endpoint.contains("86.105.224.13") {
            endpoint.replace("http://", "https://")
        } else {
            endpoint.to_string()
        };
        
        let connection_methods = if endpoint.contains("corvus-labs.io") || endpoint.contains("86.105.224.13") {
            vec![
                ("HTTP/1.1", endpoint),
                ("HTTP/2", endpoint),
                ("HTTPS", &https_url),
            ]
        } else {
            vec![
                ("HTTP/2", endpoint),
            ]
        };

        let mut success = false;
        for (method, url) in connection_methods {
            println!("  Trying {} connection...", method);
            
            let channel = match Channel::from_shared(url.to_string()) {
                Ok(channel) => channel
                    .timeout(Duration::from_secs(10))
                    .connect_timeout(Duration::from_secs(10))
                    .connect()
                    .await,
                Err(e) => {
                    println!("Failed to create channel: {}", e);
                    continue;
                }
            };

            let channel = match channel {
                Ok(ch) => ch,
                Err(e) => {
                    println!("{} connection failed: {}", method, e);
                    continue;
                }
            };

            let mut client = GeyserClient::new(channel);
            let request = tonic::Request::new(PingRequest {});
            
            match client.ping(request).await {
                Ok(response) => {
                    println!("SUCCESS: {} connection working! Response: {:?}", method, response);
                    success = true;
                    break;
                }
                Err(e) => {
                    println!("{} ping failed: {}", method, e);
                }
            }
        }
        
        if !success {
            println!("All connection methods failed for {}", endpoint);
        }
        
        println!("---");
    }

    Ok(())
} 