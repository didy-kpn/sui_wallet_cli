use crate::models::{
    alias::Alias, network_env::NetworkEnv, rpc_server::RpcServer, rpc_server_list::RpcServerList,
};
use prettytable::{cell, row, Table};
use serde_json::json;

pub struct RpcServerView {
    url: String,
    alias: String,
    env: String,
}

impl RpcServerView {
    pub fn from_rpc_server(rpc_server: &RpcServer) -> Self {
        Self {
            url: rpc_server.get_url().to_string(),
            alias: rpc_server.get_alias().to_string(),
            env: rpc_server.get_env().to_string(),
        }
    }
}

pub struct RpcServerListView(Vec<RpcServerView>);

impl RpcServerListView {
    pub fn from_rpc_server_list(
        rpc_servers: &RpcServerList,
        alias: Option<Alias>,
        env: Option<NetworkEnv>,
    ) -> Self {
        let mut servers = rpc_servers
            .get_rpc_servers()
            .iter()
            .filter(|network| {
                if let Some(alias) = &alias {
                    if !network.get_alias().contains(alias) {
                        return false;
                    }
                }

                if let Some(ref env) = env {
                    if network.get_env() != env {
                        return false;
                    }
                }
                true
            })
            .map(RpcServerView::from_rpc_server)
            .collect::<Vec<RpcServerView>>();
        servers.sort_by(|a, b| a.env.cmp(&b.env).then(a.alias.cmp(&b.alias)));
        Self(servers)
    }

    pub fn to_table(&self) -> Table {
        let mut table = Table::new();
        table.add_row(row!["Url", "Alias", "Env"]);
        for server in self.0.iter() {
            table.add_row(row![
                cell!(server.url),
                cell!(server.alias),
                cell!(server.env),
            ]);
        }
        table
    }

    pub fn to_json_string(&self) -> String {
        let json = json![{
            "rps_servers": self.0.iter().map(|server| {
                json!({
                    "rps_url": server.url,
                    "alias": server.alias,
                    "env": server.env,
                })
            }).collect::<Vec<_>>(),
        }];
        serde_json::to_string(&json).unwrap()
    }
}
