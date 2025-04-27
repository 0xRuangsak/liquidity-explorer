use anyhow::Result;
use ethers::{
    prelude::*,
    providers::{Http, Provider},
    core::types::{Address, U256},
    contract::abigen,
};
use std::str::FromStr;
use std::sync::Arc;  // Add this import

// Generate ERC20 contract bindings
abigen!(
    ERC20Contract,
    r#"[
        function name() external view returns (string)
        function symbol() external view returns (string)
        function decimals() external view returns (uint8)
        function totalSupply() external view returns (uint256)
        function balanceOf(address account) external view returns (uint256)
    ]"#,
);

#[derive(Debug, Clone)]
pub struct TokenData {
    pub address: Address,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: U256,
}

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
    
    pub async fn get_token_data(&self, token_address: &str) -> Result<TokenData> {
        let address = Address::from_str(token_address)?;
        let client = Arc::new(self.provider.clone());  // Wrap in Arc
        let contract = ERC20Contract::new(address, client);
        
        // Call contract methods to get token data
        let name = contract.name().call().await?;
        let symbol = contract.symbol().call().await?;
        let decimals = contract.decimals().call().await?;
        let total_supply = contract.total_supply().call().await?;
        
        Ok(TokenData {
            address,
            name,
            symbol,
            decimals,
            total_supply,
        })
    }

    pub async fn get_token_balance(&self, token_address: &str, wallet_address: &str) -> Result<U256> {
        let token_address = Address::from_str(token_address)?;
        let wallet_address = Address::from_str(wallet_address)?;
        
        let client = Arc::new(self.provider.clone());
        let contract = ERC20Contract::new(token_address, client);
        
        let balance = contract.balance_of(wallet_address).call().await?;
        Ok(balance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const OP_TOKEN_ADDRESS: &str = "0x4200000000000000000000000000000000000042";
    
    #[tokio::test]
    async fn test_connect_to_optimism() {
        let client = BlockchainClient::new("https://optimism.drpc.org").unwrap();
        let chain_id = client.get_chain_id().await.unwrap();
        assert_eq!(chain_id, 10);
    }
    
    #[tokio::test]
    async fn test_get_op_token_data() {
        let client = BlockchainClient::new("https://optimism.drpc.org").unwrap();
        let token_data = client.get_token_data(OP_TOKEN_ADDRESS).await.unwrap();
        
        assert_eq!(token_data.name, "Optimism");
        assert_eq!(token_data.symbol, "OP");
        assert_eq!(token_data.decimals, 18);
        assert!(token_data.total_supply > U256::zero());
    }

    #[tokio::test]
        async fn test_get_token_balance() {
            let client = BlockchainClient::new("https://optimism.drpc.org").unwrap();
            
            let wallet = "0x2A82Ae142b2e62Cb7D10b55E323ACB1Cab663a26";
            let balance = client.get_token_balance(OP_TOKEN_ADDRESS, wallet).await.unwrap();
            
            assert!(balance > U256::zero());
        }
}