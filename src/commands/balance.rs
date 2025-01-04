use super::{Command, TransactionService};
use crate::{
    error::Result,
    models::{alias_or_address::AliasOrAddress, alias_or_url::AliasOrUrl, tag_list::TagList},
    services::transaction_service::GetAllBalance,
};
use clap::Args;
use std::str::FromStr;

#[derive(Debug, Args)]
pub struct Balance {
    #[arg(value_parser = AliasOrAddress::from_str)]
    aliases_or_addresses: Vec<AliasOrAddress>,

    #[arg(short, long, value_parser = TagList::from_str)]
    tags: Option<TagList>,

    #[arg(short, long, value_parser = AliasOrUrl::from_str)]
    rpc: AliasOrUrl,

    #[arg(short, long)]
    json: bool,
}

impl<S: TransactionService<R>, R> Command<S, R> for Balance {
    fn execute(&self, service: S, repository: R) -> Result<()> {
        service.balance(
            GetAllBalance {
                aliases_or_addresses: self.aliases_or_addresses.clone(),
                tags: self.tags.clone(),
                rpc: self.rpc.clone(),
                json: self.json,
            },
            repository,
        )
    }
}
