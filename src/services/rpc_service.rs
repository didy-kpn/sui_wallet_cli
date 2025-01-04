use crate::{
    commands::{RpcService, WalletRepository},
    error::{Error, Result},
    models::{
        alias::Alias, alias_or_url::AliasOrUrl, network_env::NetworkEnv, rpc_server::RpcServer,
        rpc_url::RpcUrl, wallet_confy::WalletConfy,
    },
    views::rpc_server_view::RpcServerListView,
};
use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum AddNetworkEnv {
    Mainnet,
    Testnet,
    Devnet,
    Local,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ListNetworkEnv {
    Mainnet,
    Testnet,
    Devnet,
    Local,
    All,
}

#[derive(Default)]
pub struct RpcServiceImpl;

pub struct CreateRpc {
    pub url: RpcUrl,
    pub alias: Alias,
    pub env: Option<AddNetworkEnv>,
}

pub struct RemoveRpc {
    pub alias_or_url: AliasOrUrl,
}

pub struct ListRpc {
    pub alias: Option<Alias>,
    pub env: ListNetworkEnv,
    pub json: bool,
}

impl RpcServiceImpl {
    pub fn new() -> Self {
        Self
    }
}

impl<R: WalletRepository<WalletConfy>> RpcService<R> for RpcServiceImpl {
    fn create(&self, create_rpc: CreateRpc, repository: R) -> Result<()> {
        let mut wallet_confy = repository.load()?;

        wallet_confy.mut_rpc_servers().add(RpcServer::new(
            create_rpc.url,
            create_rpc.alias,
            match create_rpc.env {
                Some(AddNetworkEnv::Mainnet) => NetworkEnv::Mainnet,
                Some(AddNetworkEnv::Testnet) => NetworkEnv::Testnet,
                Some(AddNetworkEnv::Devnet) => NetworkEnv::Devnet,
                Some(AddNetworkEnv::Local) => NetworkEnv::Local,
                None => NetworkEnv::None,
            },
        ))?;

        repository.store(wallet_confy.clone())?;

        println!("RPC Server added successfully");
        Ok(())
    }

    fn remote(&self, remove_rpc: RemoveRpc, repository: R) -> Result<()> {
        let mut wallet_confy = repository.load()?;

        let url = match remove_rpc.alias_or_url {
            AliasOrUrl::Url(url) => url,
            AliasOrUrl::Alias(alias) => wallet_confy
                .get_rpc_servers()
                .get_url_by_alias(&alias)
                .ok_or(Error::NetworkAliasNotFound(alias))?
                .clone(),
        };

        let rpc_server = wallet_confy
            .get_rpc_servers()
            .get_by_key(&url)
            .ok_or(Error::NetworkUrlNotFound(url))?
            .clone();

        wallet_confy.mut_rpc_servers().remove(&rpc_server);

        repository.store(wallet_confy.clone())?;

        println!("RPC Server removed successfully");

        Ok(())
    }

    fn list(&self, list_rpc: ListRpc, repository: R) -> Result<()> {
        let rpc_view = RpcServerListView::from_rpc_server_list(
            repository.load()?.get_rpc_servers(),
            list_rpc.alias,
            match list_rpc.env {
                ListNetworkEnv::All => None,
                ListNetworkEnv::Mainnet => Some(NetworkEnv::Mainnet),
                ListNetworkEnv::Testnet => Some(NetworkEnv::Testnet),
                ListNetworkEnv::Devnet => Some(NetworkEnv::Devnet),
                ListNetworkEnv::Local => Some(NetworkEnv::Local),
            },
        );

        if list_rpc.json {
            println!("{}", rpc_view.to_json_string());
        } else {
            rpc_view.to_table().printstd();
        }

        Ok(())
    }
}
