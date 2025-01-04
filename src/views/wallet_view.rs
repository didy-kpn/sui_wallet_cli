use crate::models::{alias::Alias, tag_list::TagList, wallet::Wallet, wallet_list::WalletList};
use prettytable::{cell, row, Table};
use serde_json::json;

pub struct WalletView {
    address: String,
    alias: String,
    tags: String,
}

impl WalletView {
    pub fn from_wallet(wallet: &Wallet) -> Self {
        Self {
            address: wallet.get_address().to_string(),
            alias: wallet.get_alias().clone().unwrap_or_default().to_string(),
            tags: wallet.get_tags().clone().join(", "),
        }
    }
}

pub struct WalletListView(Vec<WalletView>);

impl WalletListView {
    pub fn from_walet_list(
        wallets: &WalletList,
        alias: Option<Alias>,
        tags: Option<TagList>,
    ) -> Self {
        let mut wallets = wallets
            .get_wallets()
            .iter()
            .filter(|wallet| {
                if let Some(alias) = &alias {
                    if !wallet.contains_alias(alias) {
                        return false;
                    }
                }

                if let Some(ref tags) = tags {
                    if !wallet.get_tags().contains_all(tags) {
                        return false;
                    }
                }
                true
            })
            .map(WalletView::from_wallet)
            .collect::<Vec<WalletView>>();
        wallets.sort_by(|a, b| a.address.cmp(&b.address));

        Self(wallets)
    }

    pub fn to_table(&self) -> Table {
        let mut table = Table::new();
        table.add_row(row!["Address", "Alias", "Tags"]);
        for wallet in self.0.iter() {
            table.add_row(row![
                cell!(wallet.address),
                cell!(wallet.alias),
                cell!(wallet.tags),
            ]);
        }
        table
    }

    pub fn to_json_string(&self) -> String {
        let json = json![{
            "wallets": self.0.iter().map(|wallet| {
                json!({
                    "address": wallet.address,
                    "alias": wallet.alias,
                    "tags": wallet.tags,
                })
            }).collect::<Vec<_>>(),
        }];
        serde_json::to_string(&json).unwrap()
    }
}
