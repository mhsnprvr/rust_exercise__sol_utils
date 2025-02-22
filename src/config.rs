#[derive(Debug)]
pub enum SolanaRpcUrl {
    Devnet,
    Testnet,
    Mainnet,
}

impl SolanaRpcUrl {
    pub fn as_str(&self) -> &'static str {
        match self {
            SolanaRpcUrl::Devnet => "https://api.devnet.solana.com",
            SolanaRpcUrl::Testnet => "https://api.testnet.solana.com",
            SolanaRpcUrl::Mainnet => "https://api.mainnet-beta.solana.com",
        }
    }

    pub fn from_env(env_value: &str) -> Option<Self> {
        match env_value.to_lowercase().as_str() {
            "devnet" => Some(SolanaRpcUrl::Devnet),
            "testnet" => Some(SolanaRpcUrl::Testnet),
            "mainnet" => Some(SolanaRpcUrl::Mainnet),
            _ => None,
        }
    }
}
