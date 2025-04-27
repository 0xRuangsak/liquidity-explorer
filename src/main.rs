mod clients;

use anyhow::Result;
use clients::blockchain_client::BlockchainClient;

const OP_TOKEN_ADDRESS: &str = "0x4200000000000000000000000000000000000042";

#[tokio::main]
async fn main() -> Result<()> {
    println!("Ethereum Liquidity Explorer");
    
    let client = BlockchainClient::new("https://optimism.drpc.org")?;
    let chain_id = client.get_chain_id().await?;
    println!("Connected to blockchain with chain ID: {}", chain_id);
    
    let token_data = client.get_token_data(OP_TOKEN_ADDRESS).await?;
    
    println!("\nOP Token Data:");
    println!("Address: {:?}", token_data.address);
    println!("Name: {}", token_data.name);
    println!("Symbol: {}", token_data.symbol);
    println!("Decimals: {}", token_data.decimals);
    println!("Total Supply: {}", token_data.total_supply);
    
    Ok(())
}