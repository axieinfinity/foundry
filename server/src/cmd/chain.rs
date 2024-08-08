use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChainId {
    RoninMainnet = 2020,
    RoninTestnet = 2021,
    // Add more chains as needed
}

pub struct ChainInfo {
    pub name: &'static str,
    pub rpc_url: &'static str,
}

impl ChainId {
    pub fn info(&self) -> &'static ChainInfo {
        static CHAIN_INFO: Lazy<HashMap<ChainId, ChainInfo>> = Lazy::new(|| {
            let mut m = HashMap::new();
            m.insert(
                ChainId::RoninMainnet,
                ChainInfo {
                    name: "Ronin Mainnet",
                    rpc_url: "https://api-archived.roninchain.com/rpc",
                },
            );
            m.insert(
                ChainId::RoninTestnet,
                ChainInfo {
                    name: "Ronin Testnet",
                    rpc_url: "https://saigon-archive.roninchain.com/rpc",
                },
            );
            // Add more chains as needed
            m
        });

        CHAIN_INFO.get(self).unwrap()
    }

    pub fn from_id(id: u64) -> Option<Self> {
        match id {
            2020 => Some(ChainId::RoninMainnet),
            2021 => Some(ChainId::RoninTestnet),
            // Add more matches as needed
            _ => None,
        }
    }
}
