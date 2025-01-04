use super::{Command, RpcService};
use crate::{
    error::Result,
    models::{alias::Alias, alias_or_url::AliasOrUrl, rpc_url::RpcUrl},
    services::rpc_service::{AddNetworkEnv, CreateRpc, ListNetworkEnv, ListRpc, RemoveRpc},
};
use clap::{Args, Subcommand};
use std::str::FromStr;

#[derive(Debug, Args)]
pub struct Rpc {
    #[command(subcommand)]
    pub command: RpcCommand,
}

#[derive(Debug, Subcommand, Clone)]
pub enum RpcCommand {
    Add {
        #[arg(value_parser = RpcUrl::from_str)]
        url: RpcUrl,

        #[arg(short, long, value_parser = Alias::new)]
        alias: Alias,

        #[arg(short, long)]
        env: Option<AddNetworkEnv>,
    },
    Remove {
        #[arg(value_parser = AliasOrUrl::from_str)]
        alias_or_url: AliasOrUrl,
    },
    List {
        #[arg(short, long, value_parser = Alias::new)]
        alias: Option<Alias>,

        #[arg(short, long, default_value = "all")]
        env: ListNetworkEnv,

        #[arg(short, long)]
        json: bool,
    },
}

impl<S: RpcService<R>, R> Command<S, R> for Rpc {
    fn execute(&self, service: S, repository: R) -> Result<()> {
        match self.command.clone() {
            RpcCommand::Add { url, alias, env } => {
                service.create(CreateRpc { url, alias, env }, repository)
            }
            RpcCommand::Remove { alias_or_url } => {
                service.remote(RemoveRpc { alias_or_url }, repository)
            }
            RpcCommand::List { alias, env, json } => {
                service.list(ListRpc { alias, env, json }, repository)
            }
        }
    }
}
