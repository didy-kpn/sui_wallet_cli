use super::coin_object::CoinObject;
use std::collections::HashMap;
use sui_sdk::rpc_types::{Coin, SuiCoinMetadata};

#[derive(Default, Clone)]
pub struct CoinObjectList(HashMap<String, CoinObject>);

impl CoinObjectList {
    pub fn entry(&mut self, coin: Coin, metadata: SuiCoinMetadata) {
        match self.0.get_mut(&coin.coin_type) {
            Some(objects) => {
                objects.add_object(coin);
            }
            None => {
                self.0.insert(
                    coin.coin_type.clone(),
                    CoinObject::new(metadata, vec![coin]),
                );
            }
        }
    }

    pub fn get(&self, coin_type: &str) -> Option<&CoinObject> {
        self.0.get(coin_type)
    }

    pub fn get_coin_objects(&self) -> Vec<CoinObject> {
        self.0.values().cloned().collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &CoinObject)> {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::CoinObjectList;
    use sui_json_rpc_types::{Coin, SuiCoinMetadata};
    use sui_sdk::types::{
        base_types::{ObjectID, SequenceNumber},
        digests::{ObjectDigest, TransactionDigest},
    };

    #[test]
    fn test_coin_object_list() {
        let mut list = CoinObjectList::default();
        assert_eq!(list.get_coin_objects().len(), 0);

        let dummy_metadata = SuiCoinMetadata {
            decimals: 1,
            name: "dummy".to_string(),
            symbol: "DUM".to_string(),
            description: "dummy".to_string(),
            icon_url: None,
            id: None,
        };

        let dummy_coin = Coin {
            coin_type: "dummy".to_string(),
            coin_object_id: ObjectID::random(),
            version: SequenceNumber::new(),
            digest: ObjectDigest::new([0; 32]),
            balance: 100,
            previous_transaction: TransactionDigest::default(),
        };

        list.entry(dummy_coin.clone(), dummy_metadata.clone());

        assert_eq!(list.get_coin_objects().len(), 1);
        assert!(list.get("dummy").is_some());
        assert!(list.get("dummy_not_found").is_none());
    }
}
