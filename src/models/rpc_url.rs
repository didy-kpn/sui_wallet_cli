use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use url::Url;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct RpcUrl(String);

impl FromStr for RpcUrl {
    type Err = Error;

    fn from_str(url: &str) -> Result<Self, Self::Err> {
        Url::parse(url)?;

        Ok(Self(url.to_string()))
    }
}

impl fmt::Display for RpcUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::RpcUrl;
    use crate::error::Error;
    use std::str::FromStr;

    #[test]
    fn test_from_str() {
        let url = "http://localhost:8545";
        let rpc_url = RpcUrl::from_str(url);
        assert!(rpc_url.is_ok());
        assert_eq!(rpc_url.unwrap().to_string(), url);
    }

    #[test]
    fn test_invalid() {
        assert!(matches!(
            RpcUrl::from_str("not_a_url"),
            Err(Error::UrlParseError(_))
        ));
    }
}
