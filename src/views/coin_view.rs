use prettytable::{cell, row, Table};
use serde_json::json;

use crate::models::coin_object_list::CoinObjectList;

pub struct ObjectView {
    object_id: String,
    amount: String,
    balance: String,
}

pub struct CoinView {
    name: String,
    kind: String,
    symbol: String,
    total_balance: String,
    amount: String,
    objects: Vec<ObjectView>,
    object_counts: String,
}

pub struct CoinListView(Vec<CoinView>);

impl CoinListView {
    pub fn from_coin_object_list(coins: CoinObjectList) -> Self {
        Self(
            coins
                .iter()
                .filter(|(_, coin)| {
                    0 < coin
                        .get_objects()
                        .iter()
                        .map(|object| object.balance)
                        .sum::<u64>()
                })
                .map(|(coin_type, coin)| {
                    let total_balance = coin
                        .get_objects()
                        .iter()
                        .map(|object| object.balance)
                        .sum::<u64>();
                    let metadata = coin.get_metadata().clone();
                    CoinView {
                        name: metadata.name,
                        kind: coin_type.to_string(),
                        symbol: metadata.symbol,
                        total_balance: total_balance.to_string(),
                        amount: format!(
                            "{}",
                            total_balance as f64
                                / (10_f64.powi(coin.get_metadata().decimals as i32))
                        ),
                        objects: coin
                            .get_objects()
                            .iter()
                            .map(|object| ObjectView {
                                object_id: object.coin_object_id.to_string(),
                                amount: format!(
                                    "{}",
                                    object.balance as f64 / (10_f64.powi(metadata.decimals as i32))
                                ),
                                balance: object.balance.to_string(),
                            })
                            .collect(),
                        object_counts: coin.get_objects().len().to_string(),
                    }
                })
                .collect(),
        )
    }

    pub fn to_table(&self) -> Table {
        let mut table = Table::new();
        table.add_row(row!["Name", "Type", "Amount", "Balance", "Objects"]);
        for coin in self.0.iter() {
            table.add_row(row![
                cell!(coin.name),
                cell!(coin.kind),
                cell!(format!("{} {}", coin.amount, coin.symbol)),
                cell!(coin.total_balance),
                cell!(coin.object_counts),
            ]);
        }
        table
    }

    pub fn to_json_string(&self) -> String {
        let json = json![{
            "coins": self.0.iter()
                .map(|coin| {
                    json!({
                        "name": coin.name,
                        "kind": coin.kind,
                        "amount": coin.amount,
                        "symbol":  coin.symbol,
                        "balance": coin.total_balance,
                        "objects": coin.objects.iter().map(|object| json![{
                                "object_id": object.object_id,
                                "amount": object.amount,
                                "balance": object.balance,
                        }]).collect::<Vec<_>>(),
                    })
                })
                .collect::<Vec<_>>(),
        }];
        serde_json::to_string(&json).unwrap()
    }
}
