use super::cipher::RandomlyGenerable;
use crate::error::Error;
use chacha20poly1305::Nonce;
use rand::Rng;
use serde::Deserialize;
use std::fmt;
use std::str::FromStr;

const NONCE_LEN: usize = 12;

#[derive(Debug)]
pub struct CipherNonce([u8; NONCE_LEN]);

impl CipherNonce {
    pub fn get_chacha20poly1305(&self) -> &Nonce {
        chacha20poly1305::Nonce::from_slice(&self.0)
    }
}

impl RandomlyGenerable for CipherNonce {
    fn generate_random() -> Self {
        let mut rng = rand::thread_rng();
        Self(rng.gen())
    }
}

impl Default for CipherNonce {
    fn default() -> Self {
        Self::generate_random()
    }
}

impl fmt::Display for CipherNonce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl FromStr for CipherNonce {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let bytes = hex::decode(s)?;
        if bytes.len() != NONCE_LEN {
            return Err(Error::CipherError(format!(
                "Cipher nonce must be exactly {} bytes",
                NONCE_LEN
            )));
        }
        let mut arr = [0u8; NONCE_LEN];
        arr.copy_from_slice(&bytes);
        Ok(Self(arr))
    }
}

impl<'de> Deserialize<'de> for CipherNonce {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::CipherNonce;
    use crate::error::Error;
    use crate::models::cipher::RandomlyGenerable;
    use serde_json;
    use std::str::FromStr;

    #[test]
    fn test_cipher_nonce() {
        let key1 = CipherNonce::generate_random();
        let key2 = CipherNonce::default();

        assert_ne!(key1.0, key2.0);
        assert_eq!(key1.0.len(), 12);
        assert_eq!(key2.0.len(), 12);
        assert_eq!(key1.get_chacha20poly1305().len(), 12);

        assert_eq!(
            hex::encode(CipherNonce::from_str(&key1.to_string()).unwrap().0),
            key1.to_string()
        );

        assert_eq!(
            format!("{}", CipherNonce([0u8; 12])),
            "000000000000000000000000"
        );

        assert_eq!(
            hex::encode(
                serde_json::from_str::<CipherNonce>(&format!("\"{}\"", &key1.to_string()))
                    .unwrap()
                    .0
            ),
            key1.to_string()
        );
    }

    #[test]
    fn test_from_str_invalid_length() {
        assert!(matches!(
            CipherNonce::from_str("000102030"),
            Err(Error::HexError(_))
        ),);
    }

    #[test]
    fn test_from_str_invalid_hex() {
        assert!(matches!(
            CipherNonce::from_str("0001020304"),
            Err(Error::CipherError(_))
        ));
    }

    #[test]
    fn test_deserialize_invalid() {
        let json = "\"invalid_hex_string\"";
        let result: Result<CipherNonce, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
