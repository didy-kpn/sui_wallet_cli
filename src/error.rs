use crate::models::{alias::Alias, rpc_url::RpcUrl};
use sui_sdk::types::{base_types::SuiAddress, error::SuiError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to generate new keypair: {0}")]
    GenerateNewKeyError(String),

    #[error("Name must be {0} characters or less")]
    NameTooLong(usize),

    #[error("Tag Name must only contain {0}")]
    NameInvalidCharacters(String),

    #[error("Cipher Error: {0}")]
    CipherError(String),

    #[error("Cipher key and nonce not found")]
    CipherKeyAndNonceNotFound,

    #[error("Cipher crypto error: {0}")]
    CipherCryptoError(chacha20poly1305::aead::Error),

    #[error("Hex Error: {0}")]
    HexError(#[from] hex::FromHexError),

    #[error("Envy Error: {0}")]
    EnvyError(#[from] envy::Error),

    #[error("Eyre Error: {0}")]
    EyreReportError(#[from] eyre::Report),

    #[error("Wallet with address {0} already exists")]
    WalletAddressAlreadyExists(SuiAddress),

    #[error("Wallet with alias {0} already exists")]
    WalletAliasAlreadyExists(Alias),

    #[error("Wallet with alias {0} not found")]
    WalletAliasNotFound(Alias),

    #[error("Wallet with address {0} not found")]
    WalletAddressNotFound(SuiAddress),

    #[error("Invalid Alias or Address: {0}")]
    InvalidAliasOrAddress(String),

    #[error("Invalid Alias or URL: {0}")]
    InvalidAliasOrURL(String),

    #[error("Config Error: {0}")]
    ConfigError(#[from] confy::ConfyError),

    #[error("Tag not found")]
    TagNotFound,

    #[error("Mnemonic Error: {0}")]
    MnemonicError(#[from] bip39::ErrorKind),

    #[error("Dialoguer Error: {0}")]
    DialoguerError(#[from] dialoguer::Error),

    #[error("Import address mismatch with private_key from mnemonic")]
    ImportAddressMismatchError,

    #[error("Sui Error: {0}")]
    SuiError(#[from] SuiError),

    #[error("Sui Client Error: {0}")]
    SuiClientError(#[from] sui_sdk::error::Error),

    #[error("Url ParseError: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Network Address {0} already exists")]
    NetworkAddressAlreadyExists(RpcUrl),

    #[error("Network Alias {0} already exists")]
    NetworkAliasAlreadyExists(Alias),

    #[error("Network URL {0} not found")]
    NetworkUrlNotFound(RpcUrl),

    #[error("Network Aliass {0} not found")]
    NetworkAliasNotFound(Alias),

    #[error("Reqwest Error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
