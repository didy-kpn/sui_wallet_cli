use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Serialize, Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Default)]
pub struct Alias(String);

impl fmt::Display for Alias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Alias {
    pub fn new(name: &str) -> Result<Self> {
        if name.len() > 20 {
            return Err(Error::NameTooLong(20));
        }

        if !name
            .chars()
            .all(|c| c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '_')
        {
            return Err(Error::NameInvalidCharacters(
                "alphabetic letters, numbers, or underscores".to_string(),
            ));
        }

        Ok(Self(name.to_string()))
    }

    pub fn contains(&self, alias: &Alias) -> bool {
        self.0.contains(&alias.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::Alias;
    use crate::error::Error;

    #[test]
    fn test_new() {
        let cases = vec!["test", "test123", "test_123", "a", "TEST", "_test_", ""];

        for case in cases {
            let alias = Alias::new(case);
            assert!(alias.is_ok());
            assert_eq!(alias.unwrap().to_string(), case);
        }
    }

    #[test]
    fn test_contains() {
        let alias1 = Alias::new("test123").unwrap();
        let alias2 = Alias::new("test").unwrap();
        let alias3 = Alias::new("123").unwrap();
        let alias4 = Alias::new("xyz").unwrap();

        assert!(alias1.contains(&alias2));
        assert!(alias1.contains(&alias3));
        assert!(!alias1.contains(&alias4));
    }

    #[test]
    fn test_name_too_long() {
        let long_name = "a".repeat(21);
        let alias = Alias::new(&long_name);
        assert!(matches!(alias, Err(Error::NameTooLong(20))));
    }

    #[test]
    fn test_invalid_characters() {
        let cases = vec![
            "test!",
            "test@123",
            "test space",
            "テスト",
            "test#",
            "test$",
        ];

        for case in cases {
            let alias = Alias::new(case);
            assert!(matches!(alias, Err(Error::NameInvalidCharacters(_))));
        }
    }
}
