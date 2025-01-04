use super::{Command, WalletService};
use crate::{
    error::Result,
    models::{alias::Alias, tag_list::TagList},
    services::wallet_service::{ImportWallet, KeyScheme},
};
use clap::{ArgGroup, Args};
use std::str::FromStr;
use sui_sdk::types::base_types::SuiAddress;

#[derive(Debug, Args)]
#[command(group(ArgGroup::new("kp").required(true).args(["mnemonic", "address"])))]
pub struct Import {
    #[arg(long, value_parser = SuiAddress::from_str, group = "kp")]
    address: Option<SuiAddress>,

    #[arg(short, long, value_parser = Alias::new)]
    alias: Option<Alias>,

    #[arg(short, long)]
    key_scheme: Option<KeyScheme>,

    #[arg(short, long, requires = "key_scheme", group = "kp")]
    mnemonic: bool,

    #[arg(short, long, value_parser = TagList::from_str)]
    tags: Option<TagList>,
}

impl<S: WalletService<R>, R> Command<S, R> for Import {
    fn execute(&self, service: S, repository: R) -> Result<()> {
        service.import(
            ImportWallet {
                address: self.address.clone(),
                alias: self.alias.clone(),
                key_scheme: self.key_scheme.clone(),
                mnemonic: self.mnemonic,
                tags: self.tags.clone(),
            },
            repository,
        )
    }
}
