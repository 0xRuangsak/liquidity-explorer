mod clients;
mod cli;

use anyhow::Result;
use clap::Parser;
use clients::blockchain_client::BlockchainClient;
use cli::{Cli, Command};

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Create blockchain client
    let client = BlockchainClient::new(&cli.rpc_url)?;
    
    // Process commands
    match &cli.command {
        Command::Info { address } => {
            println!("Fetching info for token: {}", address);
            
            let token_data = client.get_token_data(address).await?;
            
            println!("\nToken Data:");
            println!("Address: {:?}", token_data.address);
            println!("Name: {}", token_data.name);
            println!("Symbol: {}", token_data.symbol);
            println!("Decimals: {}", token_data.decimals);
            println!("Total Supply: {}", token_data.total_supply);
        },
        
        Command::Balance { token, wallet } => {
            println!("Checking balance of {} for token {}", wallet, token);
            
            let balance = client.get_token_balance(token, wallet).await?;
            let token_data = client.get_token_data(token).await?;
            
            println!("\nBalance Information:");
            println!("Token: {} ({})", token_data.name, token_data.symbol);
            println!("Wallet: {}", wallet);
            println!("Raw Balance: {}", balance);
            
            // Format the balance with proper decimals if possible
            if let Some(formatted) = format_token_amount(balance, token_data.decimals) {
                println!("Formatted Balance: {} {}", formatted, token_data.symbol);
            }
        },
    }
    
    Ok(())
}

// Helper function to format token amounts with proper decimal places
fn format_token_amount(amount: ethers::types::U256, decimals: u8) -> Option<String> {
    if decimals > 30 {
        return None; // Avoid potential overflow
    }
    
    let divisor = ethers::types::U256::from(10).pow(ethers::types::U256::from(decimals));
    
    if divisor.is_zero() {
        return None;
    }
    
    let whole_parts = amount / divisor;
    let fractional_parts = amount % divisor;
    
    let whole_str = whole_parts.to_string();
    let mut fractional_str = fractional_parts.to_string();
    
    // Pad with leading zeros if needed
    while fractional_str.len() < decimals as usize {
        fractional_str = format!("0{}", fractional_str);
    }
    
    // Trim trailing zeros
    let trimmed_fractional = fractional_str.trim_end_matches('0').to_string();
    if trimmed_fractional.is_empty() {
        return Some(whole_str);
    }
    
    Some(format!("{}.{}", whole_str, trimmed_fractional))
}