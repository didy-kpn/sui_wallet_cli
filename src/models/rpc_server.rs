use super::{alias::Alias, network_env::NetworkEnv, rpc_url::RpcUrl};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RpcServer(RpcUrl, Alias, NetworkEnv);

impl RpcServer {
    pub fn new(url: RpcUrl, alias: Alias, kind: NetworkEnv) -> Self {
        Self(url, alias, kind)
    }
    pub fn get_url(&self) -> &RpcUrl {
        &self.0
    }
    pub fn get_alias(&self) -> &Alias {
        &self.1
    }
    pub fn get_env(&self) -> &NetworkEnv {
        &self.2
    }
}

#[cfg(test)]
mod tests {
    use super::RpcServer;
    use crate::models::{alias::Alias, network_env::NetworkEnv, rpc_url::RpcUrl};
    use std::str::FromStr;

    #[test]
    fn test_server() {
        let url = RpcUrl::from_str("http://localhost:8545").unwrap();
        let alias = Alias::new("test").unwrap();
        let env = NetworkEnv::Mainnet;

        let server = RpcServer::new(url.clone(), alias.clone(), env.clone());

        assert_eq!(server.get_url(), &url);
        assert_eq!(server.get_alias(), &alias);
        assert_eq!(server.get_env(), &env);
    }
}
