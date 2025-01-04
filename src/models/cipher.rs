use super::cipher_key::CipherKey;
use super::cipher_nonce::CipherNonce;
use crate::error::{Error, Result};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::ChaCha20Poly1305;
use serde::Deserialize;
use std::fmt;

pub trait RandomlyGenerable {
    fn generate_random() -> Self;
}

#[derive(Default, Deserialize, Debug)]
struct CipherParameter {
    cipher_key: CipherKey,
    cipher_nonce: CipherNonce,
}

#[derive(Default)]
pub struct Cipher {
    context: Option<ChaCha20Poly1305>,
    parameter: CipherParameter,
}

impl fmt::Display for Cipher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n{}",
            self.parameter.cipher_key, self.parameter.cipher_nonce
        )
    }
}

impl Cipher {
    pub fn load_from_env() -> Result<Self> {
        let parameter = envy::from_env::<CipherParameter>()?;

        Ok(Self {
            context: Some(ChaCha20Poly1305::new(
                parameter.cipher_key.get_chacha20poly1305(),
            )),
            parameter,
        })
    }

    pub fn encrypt(&self, target: Vec<u8>) -> Result<Vec<u8>> {
        self.context
            .clone()
            .ok_or(Error::CipherKeyAndNonceNotFound)?
            .encrypt(
                self.parameter.cipher_nonce.get_chacha20poly1305(),
                target.as_ref(),
            )
            .map_err(Error::CipherCryptoError)
    }

    pub fn decrypt(&self, target: Vec<u8>) -> Result<Vec<u8>> {
        self.context
            .clone()
            .ok_or(Error::CipherKeyAndNonceNotFound)?
            .decrypt(
                self.parameter.cipher_nonce.get_chacha20poly1305(),
                target.as_ref(),
            )
            .map_err(Error::CipherCryptoError)
    }
}

#[cfg(test)]
mod tests {
    use super::Cipher;
    use crate::error::Error;

    #[test]
    fn test_cipher() {
        let key_str = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";
        let nonce_str = "000102030405060708090a0b";

        unsafe {
            std::env::set_var("CIPHER_KEY", key_str);
            std::env::set_var("CIPHER_NONCE", nonce_str);
        }

        let cipher = Cipher::load_from_env();
        assert!(cipher.is_ok());
        let cipher = cipher.unwrap();

        let display_string = format!("{}", cipher);

        assert!(display_string.contains(key_str));
        assert!(display_string.contains(nonce_str));

        let original_data = b"Hello, World!".to_vec();

        let encrypted = cipher.encrypt(original_data.clone()).unwrap();
        assert_ne!(encrypted, original_data);

        let decrypted = cipher.decrypt(encrypted).unwrap();
        assert_eq!(decrypted, original_data);
    }

    #[test]
    fn test_key_and_nonce_not_found() {
        let cipher = Cipher::default();

        assert!(matches!(
            cipher.encrypt(b"Hello, World!".to_vec()),
            Err(Error::CipherKeyAndNonceNotFound)
        ));
        assert!(matches!(
            cipher.decrypt(b"Hello, World!".to_vec()),
            Err(Error::CipherKeyAndNonceNotFound)
        ));
    }
}
