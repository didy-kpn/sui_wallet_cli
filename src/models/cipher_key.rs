use super::cipher::RandomlyGenerable;
use crate::error::Error;
use chacha20poly1305::Key;
use rand::Rng;
use serde::Deserialize;
use std::fmt;
use std::str::FromStr;

const KEY_LEN: usize = 32;

#[derive(Debug)]
pub struct CipherKey([u8; KEY_LEN]);

impl CipherKey {
    pub fn get_chacha20poly1305(&self) -> &Key {
        chacha20poly1305::Key::from_slice(&self.0)
    }
}

impl RandomlyGenerable for CipherKey {
    fn generate_random() -> Self {
        let mut rng = rand::thread_rng();
        Self(rng.gen())
    }
}

impl Default for CipherKey {
    fn default() -> Self {
        Self::generate_random()
    }
}

impl fmt::Display for CipherKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl FromStr for CipherKey {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let bytes = hex::decode(s)?;
        if bytes.len() != KEY_LEN {
            return Err(Error::CipherError(format!(
                "Cipher key must be exactly {} bytes",
                KEY_LEN
            )));
        }
        let mut arr = [0u8; KEY_LEN];
        arr.copy_from_slice(&bytes);
        Ok(Self(arr))
    }
}

impl<'de> Deserialize<'de> for CipherKey {
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
    use super::CipherKey;
    use crate::error::Error;
    use crate::models::cipher::RandomlyGenerable;
    use serde_json;
    use std::str::FromStr;

    #[test]
    fn test_cipher_key() {
        let key1 = CipherKey::generate_random();
        let key2 = CipherKey::default();

        assert_ne!(key1.0, key2.0);
        assert_eq!(key1.0.len(), 32);
        assert_eq!(key2.0.len(), 32);
        assert_eq!(key1.get_chacha20poly1305().len(), 32);

        assert_eq!(
            hex::encode(CipherKey::from_str(&key1.to_string()).unwrap().0),
            key1.to_string()
        );

        assert_eq!(
            format!("{}", CipherKey([0u8; 32])),
            "0000000000000000000000000000000000000000000000000000000000000000"
        );

        assert_eq!(
            hex::encode(
                serde_json::from_str::<CipherKey>(&format!("\"{}\"", &key1.to_string()))
                    .unwrap()
                    .0
            ),
            key1.to_string()
        );
    }

    #[test]
    fn test_from_str_invalid_length() {
        assert!(matches!(
            CipherKey::from_str("000102030"),
            Err(Error::HexError(_))
        ),);
    }

    #[test]
    fn test_from_str_invalid_hex() {
        assert!(matches!(
            CipherKey::from_str("0001020304"),
            Err(Error::CipherError(_))
        ));
    }

    #[test]
    fn test_deserialize_invalid() {
        let json = "\"invalid_hex_string\"";
        let result: Result<CipherKey, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
