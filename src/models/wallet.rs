use super::{alias::Alias, tag_list::TagList, wallet_credential::WalletCredentials};
use crate::error::Result;
use serde::{Deserialize, Serialize};
use sui_sdk::types::{
    base_types::SuiAddress,
    crypto::{SignatureScheme, SuiKeyPair},
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Wallet {
    address: SuiAddress,
    credentials: Option<WalletCredentials>,
    alias: Option<Alias>,
    tags: TagList,
}

impl Wallet {
    pub fn new(address: SuiAddress, alias: Option<Alias>, tags: TagList) -> Self {
        Self {
            alias,
            address,
            credentials: None,
            tags,
        }
    }

    pub fn with_credentials(
        mut self,
        key_pair: SuiKeyPair,
        scheme: SignatureScheme,
        phrase: String,
    ) -> Result<Self> {
        self.credentials = Some(WalletCredentials::new(key_pair, scheme, phrase)?);
        Ok(self)
    }

    pub fn get_address(&self) -> &SuiAddress {
        &self.address
    }

    pub fn get_tags(&self) -> &TagList {
        &self.tags
    }

    pub fn mut_tags(&mut self) -> &mut TagList {
        &mut self.tags
    }

    pub fn get_alias(&self) -> &Option<Alias> {
        &self.alias
    }

    pub fn mut_alias(&mut self) -> &mut Option<Alias> {
        &mut self.alias
    }

    pub fn contains_alias(&self, alias: &Alias) -> bool {
        self.alias.clone().map_or(false, |a| a.contains(alias))
    }

    pub fn get_key_pair(&self) -> Option<SuiKeyPair> {
        if let Some(ref credentials) = self.credentials {
            credentials.get_key_pair().ok()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Wallet;
    use crate::models::{alias::Alias, tag_list::TagList};
    use std::str::FromStr;
    use sui_keys::key_derive::generate_new_key;
    use sui_sdk::types::crypto::SignatureScheme;

    #[test]
    fn test_new_wallet() {
        let (address, key_pair, _, phrase) =
            generate_new_key(SignatureScheme::ED25519, None, Some("word24".to_string())).unwrap();

        let tag_list = TagList::default();
        let alias = Alias::new("test").unwrap();

        let mut wallet = Wallet::new(address, None, tag_list.clone());

        assert_eq!(wallet.get_address(), &address);
        assert!(wallet.get_tags().contains_all(&tag_list));
        assert!(wallet.get_alias().is_none());
        assert!(!wallet.contains_alias(&alias));
        assert!(wallet.get_key_pair().is_none());

        let tag_list = TagList::from_str("tag1,tag2").unwrap();
        *wallet.mut_tags() = tag_list.clone();
        *wallet.mut_alias() = Some(alias.clone());

        assert!(wallet.get_tags().contains_all(&tag_list));
        assert_eq!(wallet.get_alias(), &Some(alias.clone()));
        assert!(wallet.contains_alias(&alias));
        assert!(!wallet.contains_alias(&Alias::new("test2").unwrap()));

        let wallet_with_creds = wallet
            .clone()
            .with_credentials(key_pair.copy(), SignatureScheme::ED25519, phrase.clone())
            .unwrap();
        assert!(wallet.get_key_pair().is_none());
        assert_eq!(wallet_with_creds.get_key_pair(), Some(key_pair));
    }
}
