use super::alias::Alias;
use crate::error::Error;
use std::str::FromStr;
use sui_sdk::types::base_types::SuiAddress;

#[derive(Debug, Clone)]
pub enum AliasOrAddress {
    Address(SuiAddress),
    Alias(Alias),
}

impl FromStr for AliasOrAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(address) = SuiAddress::from_str(s) {
            return Ok(Self::Address(address));
        }

        if let Ok(alias) = Alias::new(s) {
            return Ok(Self::Alias(alias));
        }

        Err(Error::InvalidAliasOrAddress(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::AliasOrAddress;
    use crate::error::Error;
    use std::str::FromStr;

    #[test]
    fn test_from_str() {
        assert!(matches!(
            AliasOrAddress::from_str(
                "0x123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0"
            ),
            Ok(AliasOrAddress::Address(_))
        ));
        assert!(matches!(
            AliasOrAddress::from_str("test"),
            Ok(AliasOrAddress::Alias(_))
        ));
    }

    #[test]
    fn test_invalid() {
        assert!(matches!(
            AliasOrAddress::from_str("0x123@ "),
            Err(Error::InvalidAliasOrAddress(_))
        ));
    }
}
