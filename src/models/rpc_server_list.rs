use super::{alias::Alias, rpc_server::RpcServer, rpc_url::RpcUrl};
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct RpcServerList {
    rpc_servers: HashMap<RpcUrl, RpcServer>,
    aliasses: HashMap<Alias, RpcUrl>,
}

impl RpcServerList {
    pub fn get_url_by_alias(&self, alias: &Alias) -> Option<&RpcUrl> {
        self.aliasses.get(alias)
    }

    pub fn get_by_key(&self, url: &RpcUrl) -> Option<&RpcServer> {
        self.rpc_servers.get(url)
    }

    pub fn contains_key(&self, url: &RpcUrl) -> bool {
        self.rpc_servers.contains_key(url)
    }

    pub fn contains_alias_key(&self, alias: &Alias) -> bool {
        self.aliasses.contains_key(alias)
    }

    pub fn add(&mut self, network: RpcServer) -> Result<()> {
        let url = network.get_url().clone();
        if self.rpc_servers.contains_key(&url) {
            return Err(Error::NetworkAddressAlreadyExists(url.clone()));
        }

        let alias = network.get_alias().clone();
        if self.contains_alias_key(&alias) {
            return Err(Error::NetworkAliasAlreadyExists(alias.clone()));
        }

        self.aliasses.insert(alias.clone(), url.clone());

        self.rpc_servers.insert(url.clone(), network.clone());

        Ok(())
    }

    pub fn remove(&mut self, network: &RpcServer) {
        self.rpc_servers.remove(network.get_url());
        self.aliasses.remove(network.get_alias());
    }

    pub fn get_rpc_servers(&self) -> Vec<RpcServer> {
        self.rpc_servers.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::RpcServerList;
    use crate::models::{
        alias::Alias, network_env::NetworkEnv, rpc_server::RpcServer, rpc_url::RpcUrl,
    };
    use std::str::FromStr;

    #[test]
    fn test_server_list() {
        let mut list = RpcServerList::default();
        assert_eq!(list.get_rpc_servers().len(), 0);

        let url = RpcUrl::from_str("http://localhost:8545").unwrap();
        let alias = Alias::new("test").unwrap();
        let env = NetworkEnv::Mainnet;

        let server = RpcServer::new(url.clone(), alias.clone(), env.clone());
        list.add(server.clone()).unwrap();
        assert_eq!(list.get_rpc_servers().len(), 1);

        {
            let server = list.get_by_key(&url);
            assert!(server.is_some());
            let server = server.unwrap();

            assert_eq!(server.get_url(), &url);
            assert_eq!(server.get_alias(), &alias);
            assert_eq!(server.get_env(), &env);
        }
        assert_eq!(list.get_url_by_alias(&alias), Some(&url));
        assert!(list.contains_key(&url));
        assert!(list.contains_alias_key(&alias));

        list.remove(&server);
        assert_eq!(list.get_rpc_servers().len(), 0);
        assert!(list.get_by_key(&url).is_none());
        assert!(list.get_url_by_alias(&alias).is_none());
        assert!(!list.contains_key(&url));
        assert!(!list.contains_alias_key(&alias));
    }
}
