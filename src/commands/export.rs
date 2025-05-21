use super::{Command, WalletService};
use crate::{
    error::Result,
    models::alias_or_address::AliasOrAddress,
    services::wallet_service::ExportWallet, // Corrected import path
};
use clap::Args;

#[derive(Debug, Args)]
pub struct Export {
    #[arg(value_parser = AliasOrAddress::from_str)]
    pub alias_or_address: AliasOrAddress,
}

impl<S: WalletService<R>, R> Command<S, R> for Export {
    fn execute(&self, service: S, repository: R) -> Result<()> {
        service.export(
            ExportWallet {
                alias_or_address: self.alias_or_address.clone(),
            },
            repository,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::{WalletRepository, WalletService}; // Correct import for WalletService
    use crate::error::Error;
    use crate::models::{
        alias::Alias, cipher::Cipher, // Added Cipher for test setup
        tag_list::TagList, wallet::Wallet, wallet_confy::WalletConfy,
        wallet_credential::WalletCredentials,
    };
    use crate::services::wallet_service::WalletServiceImpl; // ExportWallet is already imported above
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::str::FromStr; // For SuiAddress::from_str
    use sui_sdk::types::base_types::SuiAddress;
    // SignatureScheme is part of WalletCredential::new_for_testing implicitly

    // Mock Repository
    #[derive(Clone)]
    struct MockWalletRepository {
        wallet_confy: Rc<RefCell<WalletConfy>>,
    }

    impl MockWalletRepository {
        fn new(confy: WalletConfy) -> Self {
            Self {
                wallet_confy: Rc::new(RefCell::new(confy)),
            }
        }
    }

    impl WalletRepository<WalletConfy> for MockWalletRepository {
        fn load(&self) -> Result<WalletConfy> {
            Ok(self.wallet_confy.borrow().clone())
        }

        fn store(&self, _confy: WalletConfy) -> Result<()> {
            *self.wallet_confy.borrow_mut() = _confy;
            Ok(())
        }

        fn path(&self) -> Result<String> {
            Ok(".test_wallets.yaml".to_string())
        }
    }

    fn setup_test_cipher_env() {
        // Use fixed key and nonce for reproducible encryption/decryption in tests
        std::env::set_var("CIPHER_KEY", "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f");
        std::env::set_var("CIPHER_NONCE", "000102030405060708090a0b");
    }

    fn create_test_wallet(
        alias_str: &str,
        address_str: &str,
        phrase: Option<&str>,
    ) -> Wallet {
        let mut wallet = Wallet::new(
            SuiAddress::from_str(address_str).unwrap(),
            Some(Alias::new(alias_str.to_string()).unwrap()),
            TagList::default(),
        );

        if let Some(p) = phrase {
            setup_test_cipher_env(); // Ensure cipher is set up before creating credentials
            let creds = WalletCredentials::new_for_testing(p.to_string()).unwrap();
            wallet.set_credentials(Some(creds)); // Use the #[cfg(test)] setter
        } else {
            // Explicitly set credentials to None if no phrase
            wallet.set_credentials(None);
        }
        wallet
    }

    const TEST_ADDRESS_1: &str = "0x00000000000000000000000000000000000000000000000000000000000000a1";
    const TEST_ADDRESS_2: &str = "0x00000000000000000000000000000000000000000000000000000000000000a2";
    const TEST_ADDRESS_3: &str = "0x00000000000000000000000000000000000000000000000000000000000000a3";
    const TEST_ADDRESS_4: &str = "0x00000000000000000000000000000000000000000000000000000000000000a4";


    #[test]
    fn test_export_by_alias_success() {
        setup_test_cipher_env();
        let mut confy = WalletConfy::default();
        let test_phrase = "test mnemonic phrase correct horse battery staple";
        let wallet = create_test_wallet("testwallet", TEST_ADDRESS_1, Some(test_phrase));
        confy.add_wallet(wallet).unwrap();

        let repo = MockWalletRepository::new(confy);
        let service = WalletServiceImpl::new();
        let export_cmd = Export {
            alias_or_address: AliasOrAddress::Alias(Alias::new("testwallet".to_string()).unwrap()),
        };

        let result = export_cmd.execute(service, repo);
        assert!(result.is_ok());
        // Stdout capture is omitted for simplicity as discussed.
        // In a real scenario, we'd capture stdout or refactor service.export to return String.
    }

    #[test]
    fn test_export_by_address_success() {
        setup_test_cipher_env();
        let mut confy = WalletConfy::default();
        let test_phrase = "another test phrase for address export";
        let wallet = create_test_wallet("testwallet2", TEST_ADDRESS_2, Some(test_phrase));
        confy.add_wallet(wallet).unwrap();

        let repo = MockWalletRepository::new(confy);
        let service = WalletServiceImpl::new();
        let export_cmd = Export {
            alias_or_address: AliasOrAddress::Address(SuiAddress::from_str(TEST_ADDRESS_2).unwrap()),
        };
        let result = export_cmd.execute(service, repo);
        assert!(result.is_ok());
        // Stdout capture omitted.
    }

    #[test]
    fn test_export_wallet_not_found_alias() {
        setup_test_cipher_env(); // Still needed if any part of the path tries to load/use cipher
        let confy = WalletConfy::default(); // Empty confy
        let repo = MockWalletRepository::new(confy);
        let service = WalletServiceImpl::new();
        let export_cmd = Export {
            alias_or_address: AliasOrAddress::Alias(Alias::new("nonexistent".to_string()).unwrap()),
        };
        let result = export_cmd.execute(service, repo);
        assert!(matches!(result, Err(Error::WalletAliasNotFound(_))));
    }

    #[test]
    fn test_export_wallet_not_found_address() {
        setup_test_cipher_env();
        let confy = WalletConfy::default(); // Empty confy
        let repo = MockWalletRepository::new(confy);
        let service = WalletServiceImpl::new();
        let export_cmd = Export {
            alias_or_address: AliasOrAddress::Address(SuiAddress::from_str(TEST_ADDRESS_3).unwrap()),
        };
        let result = export_cmd.execute(service, repo);
        assert!(matches!(result, Err(Error::WalletAddressNotFound(_))));
    }

    #[test]
    fn test_export_wallet_no_mnemonic() {
        setup_test_cipher_env();
        let mut confy = WalletConfy::default();
        // Wallet with no phrase
        let wallet = create_test_wallet("no_mnemonic_wallet", TEST_ADDRESS_4, None);
        confy.add_wallet(wallet).unwrap();

        let repo = MockWalletRepository::new(confy);
        let service = WalletServiceImpl::new();
        let export_cmd = Export {
            alias_or_address: AliasOrAddress::Address(SuiAddress::from_str(TEST_ADDRESS_4).unwrap()),
        };
        let result = export_cmd.execute(service, repo);
        assert!(matches!(result, Err(Error::MnemonicNotFoundError)));
    }
}
