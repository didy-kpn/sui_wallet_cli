use std::str::FromStr;

use crate::error::Error;

use super::{alias::Alias, rpc_url::RpcUrl};

#[derive(Debug, Clone)]
pub enum AliasOrUrl {
    Url(RpcUrl),
    Alias(Alias),
}

impl FromStr for AliasOrUrl {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(url) = RpcUrl::from_str(s) {
            return Ok(Self::Url(url));
        }

        if let Ok(alias) = Alias::new(s) {
            return Ok(Self::Alias(alias));
        }

        Err(Error::InvalidAliasOrURL(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::AliasOrUrl;
    use crate::error::Error;
    use std::str::FromStr;

    #[test]
    fn test_from_str() {
        assert!(matches!(
            AliasOrUrl::from_str("http://localhost:8545"),
            Ok(AliasOrUrl::Url(_))
        ));
        assert!(matches!(
            AliasOrUrl::from_str("test"),
            Ok(AliasOrUrl::Alias(_))
        ));
    }

    #[test]
    fn test_invalid() {
        assert!(matches!(
            AliasOrUrl::from_str("0x123@ "),
            Err(Error::InvalidAliasOrURL(_))
        ));
    }
}
