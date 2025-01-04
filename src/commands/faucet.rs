use super::{Command, TransactionService};
use crate::{
    error::Result,
    models::alias_or_address::AliasOrAddress,
    services::transaction_service::{FaucetNetworkEnv, RequestFaucet},
};
use clap::Args;
use std::str::FromStr;

#[derive(Debug, Args)]
pub struct Faucet {
    #[arg(value_parser = AliasOrAddress::from_str)]
    alias_or_address: AliasOrAddress,

    #[arg(short, long)]
    env: FaucetNetworkEnv,
}

impl<S: TransactionService<R>, R> Command<S, R> for Faucet {
    fn execute(&self, service: S, repository: R) -> Result<()> {
        service.faucet(
            RequestFaucet {
                alias_or_address: self.alias_or_address.clone(),
                env: self.env.clone(),
            },
            repository,
        )
    }
}
