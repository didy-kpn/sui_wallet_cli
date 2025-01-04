use super::{Command, WalletService};
use crate::{
    error::Result,
    models::{alias::Alias, tag_list::TagList},
    services::wallet_service::ListWallet,
};
use clap::Args;
use std::str::FromStr;

#[derive(Debug, Args)]
pub struct List {
    #[arg(short, long, value_parser = Alias::new)]
    alias: Option<Alias>,

    #[arg(short, long, value_parser = TagList::from_str)]
    tags: Option<TagList>,

    #[arg(short, long)]
    json: bool,
}

impl<S: WalletService<R>, R> Command<S, R> for List {
    fn execute(&self, service: S, repository: R) -> Result<()> {
        service.list(
            ListWallet {
                alias: self.alias.clone(),
                tags: self.tags.clone(),
                json: self.json,
            },
            repository,
        )
    }
}
