use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about = "Ethereum Liquidity Explorer - Analyze token liquidity and holders")]
pub struct Cli {
    /// RPC endpoint URL
    #[arg(short, long, default_value = "https://optimism.drpc.org")]
    pub rpc_url: String,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Get basic token information
    Info {
        /// Token address
        #[arg(short, long)]
        address: String,
    },
    
    /// Check token balance for an address
    Balance {
        /// Token address
        #[arg(short, long)]
        token: String,
        
        /// Wallet address to check
        #[arg(short, long)]
        wallet: String,
    },
}