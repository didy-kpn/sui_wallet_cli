use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tag(String);

impl Tag {
    pub fn new(name: &str) -> Result<Self> {
        if name.len() > 10 {
            return Err(Error::NameTooLong(10));
        }

        if !name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        {
            return Err(Error::NameInvalidCharacters(
                "lowercase letters, numbers, or underscores".to_string(),
            ));
        }

        Ok(Self(name.to_string()))
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Tag;
    use crate::error::Error;

    #[test]
    fn test_valid_tags() {
        let tag = Tag::new("valid");
        assert!(tag.is_ok());
        assert_eq!(tag.unwrap().to_string(), "valid");
    }

    #[test]
    fn test_invalid_length() {
        assert!(matches!(
            Tag::new("too_long_name"),
            Err(Error::NameTooLong(10))
        ));
    }

    #[test]
    fn test_invalid_characters() {
        let invalid_inputs = vec![
            "UPPERCASE",
            "with space",
            "spcial!chr",
            "with-hyphn",
            "日本語",
            "@symbol",
        ];

        for input in invalid_inputs {
            assert!(matches!(
                Tag::new(input),
                Err(Error::NameInvalidCharacters(_)),
            ),);
        }
    }
}
