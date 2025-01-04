use super::{alias::Alias, tag_list::TagList, wallet::Wallet};
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sui_sdk::types::base_types::SuiAddress;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct WalletList {
    wallets: HashMap<SuiAddress, Wallet>,
    aliasses: HashMap<Alias, SuiAddress>,
}

impl WalletList {
    pub fn get_address_by_alias(&self, alias: &Alias) -> Option<&SuiAddress> {
        self.aliasses.get(alias)
    }

    pub fn get_by_key(&self, address: &SuiAddress) -> Option<&Wallet> {
        self.wallets.get(address)
    }

    pub fn contains_key(&self, address: &SuiAddress) -> bool {
        self.wallets.contains_key(address)
    }

    pub fn contains_alias_key(&self, alias: &Alias) -> bool {
        self.aliasses.contains_key(alias)
    }

    pub fn add(&mut self, wallet: Wallet) -> Result<()> {
        if self.wallets.contains_key(wallet.get_address()) {
            return Err(Error::WalletAddressAlreadyExists(*wallet.get_address()));
        }

        if let Some(ref alias) = wallet.get_alias() {
            if self.contains_alias_key(alias) {
                return Err(Error::WalletAliasAlreadyExists(alias.clone()));
            }

            self.aliasses.insert(alias.clone(), *wallet.get_address());
        }

        self.wallets.insert(*wallet.get_address(), wallet.clone());

        Ok(())
    }

    pub fn edit(
        &mut self,
        address: SuiAddress,
        alias: Option<Alias>,
        tags: Option<TagList>,
    ) -> Result<()> {
        if let Some(wallet) = self.wallets.get_mut(&address) {
            *wallet.mut_tags() = match tags {
                Some(tags) => tags,
                None => wallet.get_tags().clone(),
            };

            if let Some(alias) = alias {
                if let Some(ref old_alias) = wallet.get_alias() {
                    self.aliasses.remove(old_alias);
                }
                self.aliasses.insert(alias.clone(), *wallet.get_address());

                *wallet.mut_alias() = Some(alias.clone());
            }

            Ok(())
        } else {
            Err(Error::WalletAddressNotFound(address))
        }
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut Wallet> {
        self.wallets.values_mut()
    }

    pub fn get_wallets(&self) -> Vec<Wallet> {
        self.wallets.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::WalletList;
    use crate::models::alias::Alias;
    use crate::models::tag_list::TagList;
    use crate::models::wallet::Wallet;
    use std::str::FromStr;
    use sui_keys::key_derive::generate_new_key;
    use sui_sdk::types::crypto::SignatureScheme;

    #[test]
    fn test_new_wallet_list() {
        let mut list = WalletList::default();
        assert!(list.get_wallets().is_empty());

        let (address, _, _, _) =
            generate_new_key(SignatureScheme::ED25519, None, Some("word24".to_string())).unwrap();

        let wallet = Wallet::new(
            address,
            Some(Alias::new("test1").unwrap()),
            TagList::default(),
        );

        assert!(list.add(wallet.clone()).is_ok());
        assert_eq!(list.get_wallets().len(), 1);
        assert!(list.get_by_key(&address).is_some());
        assert!(list
            .get_by_key(&address)
            .unwrap()
            .contains_alias(&Alias::new("test1").unwrap()));
        assert!(list
            .get_by_key(&address)
            .unwrap()
            .get_tags()
            .contains_all(&TagList::default()));
        assert!(list.contains_key(&address));
        assert!(list.contains_alias_key(&Alias::new("test1").unwrap()));

        assert!(list
            .edit(
                address,
                Some(Alias::new("test2").unwrap()),
                Some(TagList::from_str("tag1,tag2").unwrap())
            )
            .is_ok());
        assert_eq!(list.get_wallets().len(), 1);
        assert!(list.get_by_key(&address).is_some());
        assert!(list
            .get_by_key(&address)
            .unwrap()
            .contains_alias(&Alias::new("test2").unwrap()));
        assert!(list
            .get_by_key(&address)
            .unwrap()
            .get_tags()
            .contains_all(&TagList::from_str("tag1,tag2").unwrap()));
        assert!(list.contains_alias_key(&Alias::new("test2").unwrap()));
    }
}
