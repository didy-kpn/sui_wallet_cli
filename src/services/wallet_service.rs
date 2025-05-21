use crate::{
    commands::{WalletRepository, WalletService},
    error::{Error, Result},
    models::{
        alias::Alias, alias_or_address::AliasOrAddress, tag_list::TagList, wallet::Wallet,
        wallet_confy::WalletConfy,
    },
    views::wallet_view::WalletListView,
};
use bip39::{Language, Mnemonic, Seed};
use clap::ValueEnum;
use dialoguer::Input;
use sui_keys::key_derive::{derive_key_pair_from_path, generate_new_key};
use sui_sdk::types::{base_types::SuiAddress, crypto::SignatureScheme};

#[derive(Default)]
pub struct WalletServiceImpl;

#[derive(ValueEnum, Debug, Clone)]
pub enum KeyScheme {
    ED25519,
    Secp256k1,
    Secp256r1,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum WordLength {
    Word12,
    Word15,
    Word18,
    Word21,
    Word24,
}

pub struct CreateWallet {
    pub alias: Option<Alias>,
    pub key_scheme: KeyScheme,
    pub mnemonic_length: WordLength,
    pub tags: Option<TagList>,
}

pub struct ImportWallet {
    pub address: Option<SuiAddress>,
    pub alias: Option<Alias>,
    pub key_scheme: Option<KeyScheme>,
    pub mnemonic: bool,
    pub tags: Option<TagList>,
}

pub struct EditWallet {
    pub alias_or_address: AliasOrAddress,
    pub alias: Option<Alias>,
    pub tags: Option<TagList>,
}

pub struct ListWallet {
    pub alias: Option<Alias>,
    pub tags: Option<TagList>,
    pub json: bool,
}

pub struct ExportWallet {
    pub alias_or_address: AliasOrAddress,
}

impl WalletServiceImpl {
    pub fn new() -> Self {
        Self
    }
}

impl<R: WalletRepository<WalletConfy>> WalletService<R> for WalletServiceImpl {
    fn create(&self, create_wallet: CreateWallet, repository: R) -> Result<()> {
        let mut wallet_confy = repository.load()?;

        let (alias, key_scheme, mnemonic_length, tags) = (
            create_wallet.alias.clone(),
            create_wallet.key_scheme.clone(),
            create_wallet.mnemonic_length.clone(),
            create_wallet.tags.unwrap_or_default().clone(),
        );

        let (address, key_pair, scheme, phrase) = generate_new_key(
            match key_scheme {
                KeyScheme::ED25519 => SignatureScheme::ED25519,
                KeyScheme::Secp256k1 => SignatureScheme::Secp256k1,
                KeyScheme::Secp256r1 => SignatureScheme::Secp256r1,
            },
            None,
            Some(
                match mnemonic_length {
                    WordLength::Word12 => "word12",
                    WordLength::Word15 => "word15",
                    WordLength::Word18 => "word18",
                    WordLength::Word21 => "word21",
                    WordLength::Word24 => "word24",
                }
                .to_string(),
            ),
        )
        .map_err(|e| Error::GenerateNewKeyError(e.to_string()))?;

        if !wallet_confy.get_tags().contains_all(&tags) {
            return Err(Error::TagNotFound);
        }

        wallet_confy.add_wallet(
            Wallet::new(address, alias, tags.clone()).with_credentials(key_pair, scheme, phrase)?,
        )?;

        repository.store(wallet_confy.clone())?;

        println!("Wallet created successfully");
        println!("Alias: {}", create_wallet.alias.unwrap_or_default());
        println!("Address: {}", address);

        Ok(())
    }

    fn import(&self, import_wallet: ImportWallet, repository: R) -> Result<()> {
        let mut wallet_confy = repository.load()?;

        if import_wallet.mnemonic {
            let mnemonic = Mnemonic::from_phrase(
                &Input::<String>::new()
                    .with_prompt("Enter mnemonic phrase")
                    .interact_text()?,
                Language::English,
            )?;

            let scheme = match import_wallet.key_scheme.unwrap() {
                KeyScheme::ED25519 => SignatureScheme::ED25519,
                KeyScheme::Secp256k1 => SignatureScheme::Secp256k1,
                KeyScheme::Secp256r1 => SignatureScheme::Secp256r1,
            };

            let seed = Seed::new(&mnemonic, "");
            let (address, key_pair) = derive_key_pair_from_path(seed.as_bytes(), None, &scheme)?;

            if import_wallet.address.is_some() && import_wallet.address.unwrap() != address {
                return Err(Error::ImportAddressMismatchError);
            }

            wallet_confy.add_wallet(
                Wallet::new(
                    address,
                    import_wallet.alias,
                    import_wallet.tags.unwrap_or_default(),
                )
                .with_credentials(
                    key_pair,
                    scheme,
                    mnemonic.phrase().to_string(),
                )?,
            )?;
        } else if let Some(address) = import_wallet.address {
            wallet_confy.add_wallet(Wallet::new(
                address,
                import_wallet.alias,
                import_wallet.tags.unwrap_or_default(),
            ))?;
        }

        println!("Wallet import successfully");

        Ok(())
    }

    fn edit(&self, edit_wallet: EditWallet, repository: R) -> Result<()> {
        let mut wallet_confy = repository.load()?;
        let wallets = wallet_confy.mut_wallets();

        match edit_wallet.alias_or_address {
            AliasOrAddress::Address(address) => {
                wallets.edit(address, edit_wallet.alias, edit_wallet.tags)?;
            }
            AliasOrAddress::Alias(alias) => {
                if let Some(address) = wallets.get_address_by_alias(&alias) {
                    wallets.edit(*address, edit_wallet.alias, edit_wallet.tags)?;
                } else {
                    return Err(Error::WalletAliasNotFound(alias));
                }
            }
        };

        repository.store(wallet_confy.clone())?;

        println!("Wallet edited successfully");

        Ok(())
    }

    fn list(&self, list_wallet: ListWallet, repository: R) -> Result<()> {
        let wallet_view = WalletListView::from_walet_list(
            repository.load()?.get_wallets(),
            list_wallet.alias,
            list_wallet.tags,
        );

        if list_wallet.json {
            println!("{}", wallet_view.to_json_string());
        } else {
            wallet_view.to_table().printstd();
        }

        Ok(())
    }

    fn export(&self, export_wallet: ExportWallet, repository: R) -> Result<()> {
        let wallet_confy = repository.load()?;
        let wallets = wallet_confy.get_wallets(); // Assuming get_wallets() returns a reference to WalletList or similar

        let wallet_to_export = match export_wallet.alias_or_address {
            AliasOrAddress::Address(address) => wallets.get_wallet_by_address(&address),
            AliasOrAddress::Alias(alias) => wallets.get_wallet_by_alias(&alias),
        };

        match wallet_to_export {
            Some(wallet) => {
                if let Some(phrase) = wallet.get_phrase() { // Assuming Wallet has a get_phrase() method that returns Option<&String>
                    println!("{}", phrase);
                    Ok(())
                } else {
                    Err(Error::MnemonicNotFoundError) // Need to define this error variant
                }
            }
            None => match export_wallet.alias_or_address {
                AliasOrAddress::Address(addr) => Err(Error::WalletAddressNotFound(addr)), // Assuming this error variant exists
                AliasOrAddress::Alias(alias) => Err(Error::WalletAliasNotFound(alias)), // Assuming this error variant exists
            },
        }
    }
}
