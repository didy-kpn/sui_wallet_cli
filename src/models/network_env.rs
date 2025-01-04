use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt, str::FromStr};

#[derive(Deserialize, Serialize, Debug, Clone, Eq)]
pub enum NetworkEnv {
    Mainnet,
    Testnet,
    Devnet,
    Local,
    None,
}

impl fmt::Display for NetworkEnv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkEnv::Mainnet => write!(f, "mainnet"),
            NetworkEnv::Testnet => write!(f, "testnet"),
            NetworkEnv::Devnet => write!(f, "devnet"),
            NetworkEnv::Local => write!(f, "local"),
            NetworkEnv::None => write!(f, "-"),
        }
    }
}

impl FromStr for NetworkEnv {
    type Err = String;

    fn from_str(kind: &str) -> Result<Self, Self::Err> {
        match kind.to_lowercase().as_str() {
            "mainnet" => Ok(NetworkEnv::Mainnet),
            "testnet" => Ok(NetworkEnv::Testnet),
            "devnet" => Ok(NetworkEnv::Devnet),
            "local" => Ok(NetworkEnv::Local),
            "none" => Ok(NetworkEnv::None),
            "-" => Ok(NetworkEnv::None),
            _ => Err(format!("Unknown network env: {}", kind)),
        }
    }
}

impl PartialEq for NetworkEnv {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl PartialOrd for NetworkEnv {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NetworkEnv {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_value = match self {
            NetworkEnv::Mainnet => 0,
            NetworkEnv::Testnet => 1,
            NetworkEnv::Devnet => 2,
            NetworkEnv::Local => 3,
            NetworkEnv::None => 4,
        };

        let other_value = match other {
            NetworkEnv::Mainnet => 0,
            NetworkEnv::Testnet => 1,
            NetworkEnv::Devnet => 2,
            NetworkEnv::Local => 3,
            NetworkEnv::None => 4,
        };

        self_value.cmp(&other_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_network_env() {
        assert_eq!(NetworkEnv::Mainnet.to_string(), "mainnet");
        assert_eq!(NetworkEnv::Testnet.to_string(), "testnet");
        assert_eq!(NetworkEnv::Devnet.to_string(), "devnet");
        assert_eq!(NetworkEnv::Local.to_string(), "local");
        assert_eq!(NetworkEnv::None.to_string(), "-");

        assert_eq!(
            NetworkEnv::from_str("mainnet").unwrap(),
            NetworkEnv::Mainnet
        );
        assert_eq!(
            NetworkEnv::from_str("MAINNET").unwrap(),
            NetworkEnv::Mainnet
        );
        assert_eq!(
            NetworkEnv::from_str("testnet").unwrap(),
            NetworkEnv::Testnet
        );
        assert_eq!(NetworkEnv::from_str("devnet").unwrap(), NetworkEnv::Devnet);
        assert_eq!(NetworkEnv::from_str("local").unwrap(), NetworkEnv::Local);
        assert_eq!(NetworkEnv::from_str("none").unwrap(), NetworkEnv::None);
        assert_eq!(NetworkEnv::from_str("-").unwrap(), NetworkEnv::None);
        assert!(NetworkEnv::from_str("invalid").is_err());

        let mut envs = vec![
            NetworkEnv::None,
            NetworkEnv::Mainnet,
            NetworkEnv::Devnet,
            NetworkEnv::Local,
            NetworkEnv::Testnet,
        ];
        envs.sort();
        assert_eq!(
            envs,
            vec![
                NetworkEnv::Mainnet,
                NetworkEnv::Testnet,
                NetworkEnv::Devnet,
                NetworkEnv::Local,
                NetworkEnv::None,
            ]
        );
    }
}
