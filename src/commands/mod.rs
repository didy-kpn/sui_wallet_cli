use crate::{
    error::Result,
    services::{
        rpc_service::{CreateRpc, ListRpc, RemoveRpc},
        tag_service::{CreateTags, ListTags, RemoveTags},
        transaction_service::{GetAllBalance, RequestFaucet},
        wallet_service::{CreateWallet, EditWallet, ExportWallet, ImportWallet, ListWallet},
    },
};

pub mod balance;
pub mod cipher;
pub mod create;
pub mod edit;
pub mod export;
pub mod faucet;
pub mod import;
pub mod list;
pub mod rpc;
pub mod tag;

pub trait Command<S, R> {
    fn execute(&self, service: S, repository: R) -> Result<()>;
}

pub trait WalletService<R> {
    fn create(&self, create_wallet: CreateWallet, repository: R) -> Result<()>;
    fn import(&self, import_wallet: ImportWallet, repository: R) -> Result<()>;
    fn edit(&self, edit_wallet: EditWallet, repository: R) -> Result<()>;
    fn list(&self, list_wallet: ListWallet, repository: R) -> Result<()>;
    fn export(&self, export_wallet: ExportWallet, repository: R) -> Result<()>;
}

pub trait TagService<R> {
    fn create(&self, create_tags: CreateTags, repository: R) -> Result<()>;
    fn remote(&self, remove_tags: RemoveTags, repository: R) -> Result<()>;
    fn list(&self, list_tags: ListTags, repository: R) -> Result<()>;
}

pub trait RpcService<R> {
    fn create(&self, create_rpc: CreateRpc, repository: R) -> Result<()>;
    fn remote(&self, remove_rpc: RemoveRpc, repository: R) -> Result<()>;
    fn list(&self, list_rpc: ListRpc, repository: R) -> Result<()>;
}

pub trait TransactionService<R> {
    fn activity(&self) -> Result<()>;
    fn balance(&self, get_all_balance: GetAllBalance, repository: R) -> Result<()>;
    fn faucet(&self, request_faucet: RequestFaucet, repository: R) -> Result<()>;
    fn send_coin(&self) -> Result<()>;
}

pub trait CipherService {
    fn create(&self) -> Result<()>;
}

pub trait WalletRepository<C> {
    fn load(&self) -> Result<C>;
    fn store(&self, confy: C) -> Result<()>;
}
