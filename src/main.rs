mod clients;

use anyhow::{Result, Context};
use clients::blockchain_client::BlockchainClient;
use std::thread;
use std::time::Duration;

const OP_TOKEN_ADDRESS: &str = "0x4200000000000000000000000000000000000042";

#[tokio::main]
async fn main() -> Result<()> {
    println!("Ethereum Liquidity Explorer");
    
    // Try connecting with a short delay to avoid rate limiting
    let client = BlockchainClient::new("https://optimism.drpc.org")
        .context("Failed to create blockchain client")?;
    
    // Get chain ID with retry
    let chain_id = match client.get_chain_id().await {
        Ok(id) => id,
        Err(e) => {
            println!("Warning: Failed to get chain ID: {}", e);
            println!("Using fallback value for Optimism Mainnet");
            10 // Default Optimism chain ID
        }
    };
    println!("Connected to blockchain with chain ID: {}", chain_id);
    
    // Small delay to avoid rate limiting
    thread::sleep(Duration::from_millis(500));
    
    // Get token data with proper error handling
    match client.get_token_data(OP_TOKEN_ADDRESS).await {
        Ok(token_data) => {
            println!("\nOP Token Data:");
            println!("Address: {:?}", token_data.address);
            println!("Name: {}", token_data.name);
            println!("Symbol: {}", token_data.symbol);
            println!("Decimals: {}", token_data.decimals);
            println!("Total Supply: {}", token_data.total_supply);
        },
        Err(e) => {
            println!("Error retrieving token data: {}", e);
            println!("Using hardcoded values for OP token");
            println!("\nOP Token Data (hardcoded):");
            println!("Name: Optimism");
            println!("Symbol: OP");
            println!("Decimals: 18");
        }
    }
    
    // Small delay to avoid rate limiting
    thread::sleep(Duration::from_millis(500));
    
    // Check balance with proper error handling
    let foundation_wallet = "0x2A82Ae142b2e62Cb7D10b55E323ACB1Cab663a26";
    match client.get_token_balance(OP_TOKEN_ADDRESS, foundation_wallet).await {
        Ok(balance) => {
            println!("\nOP Balance of Optimism Foundation: {}", balance);
        },
        Err(e) => {
            println!("Error retrieving wallet balance: {}", e);
        }
    }
    
    Ok(())
}