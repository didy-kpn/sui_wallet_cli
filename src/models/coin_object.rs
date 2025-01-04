use sui_sdk::rpc_types::{Coin, SuiCoinMetadata};

#[derive(Clone)]
pub struct CoinObject(SuiCoinMetadata, Vec<Coin>);

impl CoinObject {
    pub fn new(metadata: SuiCoinMetadata, coins: Vec<Coin>) -> Self {
        Self(metadata, coins)
    }

    pub fn get_metadata(&self) -> &SuiCoinMetadata {
        &self.0
    }

    pub fn get_objects(&self) -> &Vec<Coin> {
        &self.1
    }

    pub fn add_object(&mut self, coin: Coin) {
        self.1.push(coin);
    }

    pub fn len(&self) -> usize {
        self.1.len()
    }

    pub fn is_empty(&self) -> bool {
        self.1.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::CoinObject;
    use sui_json_rpc_types::{Coin, SuiCoinMetadata};
    use sui_sdk::types::{
        base_types::{ObjectID, SequenceNumber},
        digests::{ObjectDigest, TransactionDigest},
    };

    #[test]
    fn test_coin_object() {
        let dummy_metadata = SuiCoinMetadata {
            decimals: 1,
            name: "dummy".to_string(),
            symbol: "DUM".to_string(),
            description: "dummy".to_string(),
            icon_url: None,
            id: None,
        };
        let mut coin_object = CoinObject::new(dummy_metadata.clone(), vec![]);

        assert_eq!(coin_object.get_metadata(), &dummy_metadata);
        assert_eq!(coin_object.get_objects(), &vec![]);
        assert_eq!(coin_object.len(), 0);
        assert!(coin_object.is_empty());

        let dummy_coin = Coin {
            coin_type: "dummy".to_string(),
            coin_object_id: ObjectID::random(),
            version: SequenceNumber::new(),
            digest: ObjectDigest::new([0; 32]),
            balance: 100,
            previous_transaction: TransactionDigest::default(),
        };

        coin_object.add_object(dummy_coin.clone());

        assert_eq!(coin_object.get_metadata(), &dummy_metadata);
        assert_eq!(coin_object.get_objects(), &vec![dummy_coin]);
        assert_eq!(coin_object.len(), 1);
        assert!(!coin_object.is_empty());
    }
}
