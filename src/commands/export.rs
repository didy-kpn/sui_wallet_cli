use super::{Command, WalletService};
use crate::{
    error::Result,
    models::alias_or_address::AliasOrAddress,
    services::wallet_service::ExportWallet, // Corrected import path
};
use clap::Args;
use std::str::FromStr;

#[derive(Debug, Args)]
pub struct Export {
    #[arg(value_parser = AliasOrAddress::from_str)]
    pub alias_or_address: AliasOrAddress,
}

impl<S: WalletService<R>, R> Command<S, R> for Export {
    fn execute(&self, service: S, repository: R) -> Result<()> {
        service.export(
            ExportWallet {
                alias_or_address: self.alias_or_address.clone(),
            },
            repository,
        )
    }
}
