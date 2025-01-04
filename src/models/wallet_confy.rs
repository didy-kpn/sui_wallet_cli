use super::{rpc_server_list::RpcServerList, wallet::Wallet};
use crate::{
    error::{Error, Result},
    models::{tag_list::TagList, wallet_list::WalletList},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct WalletConfy {
    wallets: WalletList,
    tags: TagList,
    rpc_servers: RpcServerList,
}

impl WalletConfy {
    pub fn add_wallet(&mut self, wallet: Wallet) -> Result<()> {
        if !self.tags.contains_all(wallet.get_tags()) {
            return Err(Error::TagNotFound);
        }

        self.wallets.add(wallet)
    }

    pub fn get_wallets(&self) -> &WalletList {
        &self.wallets
    }

    pub fn mut_wallets(&mut self) -> &mut WalletList {
        &mut self.wallets
    }

    pub fn get_tags(&self) -> &TagList {
        &self.tags
    }

    pub fn mut_tags(&mut self) -> &mut TagList {
        &mut self.tags
    }

    pub fn get_rpc_servers(&self) -> &RpcServerList {
        &self.rpc_servers
    }

    pub fn mut_rpc_servers(&mut self) -> &mut RpcServerList {
        &mut self.rpc_servers
    }
}
