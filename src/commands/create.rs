use super::{Command, WalletService};
use crate::{
    error::Result,
    models::{alias::Alias, tag_list::TagList},
    services::wallet_service::{CreateWallet, KeyScheme, WordLength},
};
use clap::Args;
use std::str::FromStr;

#[derive(Debug, Args)]
pub struct Create {
    #[arg(short, long, value_parser = Alias::new)]
    alias: Option<Alias>,

    #[arg(short, long, default_value = "ed25519")]
    key_scheme: KeyScheme,

    #[arg(short, long, default_value = "word24")]
    mnemonic_length: WordLength,

    #[arg(short, long, value_parser = TagList::from_str)]
    tags: Option<TagList>,
}

impl<S: WalletService<R>, R> Command<S, R> for Create {
    fn execute(&self, service: S, repository: R) -> Result<()> {
        service.create(
            CreateWallet {
                alias: self.alias.clone(),
                key_scheme: self.key_scheme.clone(),
                mnemonic_length: self.mnemonic_length.clone(),
                tags: self.tags.clone(),
            },
            repository,
        )
    }
}
