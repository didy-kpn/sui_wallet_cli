use super::cipher::Cipher;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use sui_sdk::types::crypto::{PublicKey, SignatureScheme, SuiKeyPair};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WalletCredentials {
    public_key: PublicKey,
    encrypted_private_key: String,
    key_scheme: SignatureScheme,
    encrypted_mnemonic: String,
}

impl WalletCredentials {
    pub fn new(key_pair: SuiKeyPair, scheme: SignatureScheme, phrase: String) -> Result<Self> {
        let cipher = Cipher::load_from_env()?;

        Ok(Self {
            public_key: key_pair.public(),
            encrypted_private_key: hex::encode(cipher.encrypt(key_pair.to_bytes())?),
            key_scheme: scheme,
            encrypted_mnemonic: hex::encode(cipher.encrypt(phrase.into_bytes())?),
        })
    }

    pub fn get_key_pair(&self) -> Result<SuiKeyPair> {
        let cipher = Cipher::load_from_env()?;
        let private_key = cipher.decrypt(hex::decode(&self.encrypted_private_key)?)?;
        Ok(SuiKeyPair::from_bytes(&private_key)?)
    }
}

#[cfg(test)]
mod tests {
    use super::WalletCredentials;
    use sui_keys::key_derive::generate_new_key;
    use sui_sdk::types::crypto::SignatureScheme;

    #[test]
    fn test_wallet_credentials() {
        let key_str = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";
        let nonce_str = "000102030405060708090a0b";

        unsafe {
            std::env::set_var("CIPHER_KEY", key_str);
            std::env::set_var("CIPHER_NONCE", nonce_str);
        }

        let (_, key_pair, scheme, phrase) =
            generate_new_key(SignatureScheme::ED25519, None, Some("word24".to_string())).unwrap();

        let credentials = WalletCredentials::new(key_pair.copy(), scheme, phrase.clone());
        assert!(credentials.is_ok());
        let credentials = credentials.unwrap();
        assert!(credentials.get_key_pair().is_ok());
        assert_eq!(credentials.get_key_pair().unwrap(), key_pair);
    }
}
