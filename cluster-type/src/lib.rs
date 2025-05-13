#![cfg_attr(feature = "frozen-abi", feature(min_specialization))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#[cfg(feature = "frozen-abi")]
use solana_frozen_abi_macro::{AbiEnumVisitor, AbiExample};
use {solana_hash::Hash, std::str::FromStr};

// The order can't align with release lifecycle only to remain ABI-compatible...
#[cfg_attr(feature = "frozen-abi", derive(AbiExample, AbiEnumVisitor))]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Deserialize, serde_derive::Serialize)
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClusterType {
    Testnet,
    MainnetBeta,
    Devnet,
    Development,
}

impl ClusterType {
    pub const STRINGS: [&'static str; 4] = ["development", "devnet", "testnet", "mainnet-beta"];

    /// Get the known genesis hash for this ClusterType
    pub fn get_genesis_hash(&self) -> Option<Hash> {
        match self {
            Self::MainnetBeta => None,
            Self::Testnet => {
                Some(Hash::from_str("C7ucgdDEhxLTpXHhWSZxavSVmaNTUJWwT5iTdeaviDho").unwrap())
            }
            Self::Devnet => None,
            Self::Development => None,
        }
    }
}

impl FromStr for ClusterType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "development" => Ok(ClusterType::Development),
            "devnet" => Ok(ClusterType::Devnet),
            "testnet" => Ok(ClusterType::Testnet),
            "mainnet-beta" => Ok(ClusterType::MainnetBeta),
            _ => Err(format!("{s} is unrecognized for cluster type")),
        }
    }
}
