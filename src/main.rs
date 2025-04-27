mod clients;

use anyhow::Result;
use clients::blockchain_client::BlockchainClient;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Ethereum Liquidity Explorer");
    
    let client = BlockchainClient::new("https://optimism.drpc.org")?;
    let chain_id = client.get_chain_id().await?;
    println!("Connected to blockchain with chain ID: {}", chain_id);
    
    Ok(())
}