use crate::{
    commands::{TransactionService, WalletRepository},
    error::{Error, Result},
    models::{
        alias_or_address::AliasOrAddress, alias_or_url::AliasOrUrl,
        coin_object_list::CoinObjectList, rpc_url::RpcUrl, tag_list::TagList, wallet::Wallet,
        wallet_confy::WalletConfy,
    },
    views::coin_view::CoinListView,
};
use clap::ValueEnum;
use indicatif::ProgressIterator;
use serde_json::json;
use sui_sdk::SuiClientBuilder;

#[derive(ValueEnum, Clone, Debug)]
pub enum FaucetNetworkEnv {
    Testnet,
    Devnet,
    Localnet,
}

#[derive(Default)]
pub struct TransactionServiceImpl;

pub struct GetAllBalance {
    pub aliases_or_addresses: Vec<AliasOrAddress>,
    pub tags: Option<TagList>,
    pub rpc: AliasOrUrl,
    pub json: bool,
}

// #[derive(Debug, Args)]
pub struct RequestFaucet {
    // #[arg(value_parser = AliasOrAddress::from_str)]
    pub alias_or_address: AliasOrAddress,

    // #[arg(short, long)]
    pub env: FaucetNetworkEnv,
}

impl TransactionServiceImpl {
    pub fn new() -> Self {
        Self
    }

    fn get_all_balance(&self, wallets: Vec<&Wallet>, url: RpcUrl) -> Result<CoinObjectList> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let mut coin_list = CoinObjectList::default();

            let sui_client = SuiClientBuilder::default().build(&url.to_string()).await?;

            for wallet in wallets.iter().progress() {
                let mut next_cursor = None;

                loop {
                    let result = sui_client
                        .coin_read_api()
                        .get_all_coins(*wallet.get_address(), next_cursor, None)
                        .await?;

                    for coin in result.data.into_iter() {
                        let metadata = match coin_list.get(&coin.coin_type) {
                            Some(coins) => coins.get_metadata().clone(),
                            None => sui_client
                                .coin_read_api()
                                .get_coin_metadata(coin.coin_type.clone())
                                .await?
                                .unwrap(),
                        };

                        coin_list.entry(coin, metadata);
                    }

                    if result.has_next_page {
                        next_cursor = result.next_cursor;
                    } else {
                        break;
                    }
                }
            }

            Ok(coin_list)
        })
        .map_err(Error::SuiClientError)
    }
}

impl<R: WalletRepository<WalletConfy>> TransactionService<R> for TransactionServiceImpl {
    fn activity(&self) -> Result<()> {
        todo!()
    }
    fn balance(&self, get_all_balance: GetAllBalance, repository: R) -> Result<()> {
        let confy = repository.load()?;
        let wallets = confy.get_wallets();
        let rpc_servers = confy.get_rpc_servers();

        let wallets: Vec<&Wallet> = get_all_balance
            .aliases_or_addresses
            .iter()
            .filter_map(|alias_or_address| match alias_or_address {
                AliasOrAddress::Address(address) => wallets.get_by_key(address),
                AliasOrAddress::Alias(alias) => wallets
                    .get_address_by_alias(alias)
                    .and_then(|address| wallets.get_by_key(address)),
            })
            .filter(|wallet| {
                get_all_balance
                    .tags
                    .as_ref()
                    .map_or(true, |tag| wallet.get_tags().contains_all(tag))
            })
            .collect();

        let url = match get_all_balance.rpc {
            AliasOrUrl::Url(url) => url,
            AliasOrUrl::Alias(alias) => rpc_servers
                .get_url_by_alias(&alias)
                .ok_or(Error::NetworkAliasNotFound(alias))?
                .clone(),
        };

        let coin_view = CoinListView::from_coin_object_list(self.get_all_balance(wallets, url)?);

        if get_all_balance.json {
            println!("{}", coin_view.to_json_string());
        } else {
            coin_view.to_table().printstd();
        }

        Ok(())
    }

    fn faucet(&self, request_faucet: RequestFaucet, repository: R) -> Result<()> {
        let confy = repository.load()?;
        let wallet = match request_faucet.alias_or_address {
            AliasOrAddress::Address(address) => confy
                .get_wallets()
                .get_by_key(&address)
                .ok_or(Error::WalletAddressNotFound(address))?,
            AliasOrAddress::Alias(alias) => {
                let address = confy
                    .get_wallets()
                    .get_address_by_alias(&alias)
                    .ok_or(Error::WalletAliasNotFound(alias))?;

                confy
                    .get_wallets()
                    .get_by_key(address)
                    .ok_or(Error::WalletAddressNotFound(*address))?
            }
        }
        .clone();

        let resp = reqwest::blocking::Client::new()
            .post(match request_faucet.env {
                FaucetNetworkEnv::Testnet => "https://faucet.testnet.sui.io/gas",
                FaucetNetworkEnv::Devnet => "https://faucet.devnet.sui.io/gas",
                FaucetNetworkEnv::Localnet => "http://127.0.0.1:9123/gas",
            })
            .header("Content-Type", "application/json")
            .json(&json![{
                "FixedAmountRequest": {
                    "recipient": wallet.get_address(),
                }
            }])
            .send()?;

        println!("Faucet request sent successfully: {0}", resp.status());

        Ok(())
    }

    fn send_coin(&self) -> Result<()> {
        todo!()
    }
}
