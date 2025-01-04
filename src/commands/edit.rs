use crate::{
    error::Result,
    models::{alias::Alias, alias_or_address::AliasOrAddress, tag_list::TagList},
    services::wallet_service::EditWallet,
};
use clap::Args;
use std::str::FromStr;

use super::{Command, WalletService};

#[derive(Debug, Args)]
pub struct Edit {
    #[arg(value_parser = AliasOrAddress::from_str)]
    alias_or_address: AliasOrAddress,

    #[arg(short, long, value_parser = Alias::new)]
    alias: Option<Alias>,

    #[arg(short, long, value_parser = TagList::from_str)]
    tags: Option<TagList>,
}

impl<S: WalletService<R>, R> Command<S, R> for Edit {
    fn execute(&self, service: S, repository: R) -> Result<()> {
        service.edit(
            EditWallet {
                alias_or_address: self.alias_or_address.clone(),
                alias: self.alias.clone(),
                tags: self.tags.clone(),
            },
            repository,
        )
    }
}
