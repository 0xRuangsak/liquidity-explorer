use anyhow::Result;
use ethers::{
    prelude::*,
    providers::{Http, Provider},
};

pub struct BlockchainClient {
    provider: Provider<Http>,
}

impl BlockchainClient {
    pub fn new(rpc_url: &str) -> Result<Self> {
        let provider = Provider::<Http>::try_from(rpc_url)?;
        Ok(Self { provider })
    }
    
    pub async fn get_chain_id(&self) -> Result<u64> {
        let chain_id = self.provider.get_chainid().await?;
        Ok(chain_id.as_u64())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_connect_to_optimism() {
        // Try a different public endpoint that doesn't require authentication
        let client = BlockchainClient::new("https://optimism.drpc.org").unwrap();
        let chain_id = client.get_chain_id().await.unwrap();
        
        // Optimism mainnet chain ID is 10
        assert_eq!(chain_id, 10);
    }
}