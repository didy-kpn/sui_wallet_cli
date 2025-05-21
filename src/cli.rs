use crate::{
    commands::{
        balance::Balance, cipher::Cipher, create::Create, edit::Edit, export::Export,
        faucet::Faucet, import::Import, list::List, rpc::Rpc, tag::Tag, Command,
    },
    error::Error,
    models::wallet_confy::WalletConfy,
    services::{
        cipher_service::CipherServiceImpl, rpc_service::RpcServiceImpl,
        tag_service::TagServiceImpl, transaction_service::TransactionServiceImpl,
        wallet_service::WalletServiceImpl,
    },
    storages::confy::ConfyClient,
};
use clap::{Parser, Subcommand};
use std::process::ExitCode;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Balance(Balance),
    Create(Create),
    Edit(Edit),
    #[command(about = "Export a wallet's mnemonic phrase")]
    Export(Export),
    Faucet(Faucet),
    Import(Import),
    List(List),
    NewCipher(Cipher),
    Rpc(Rpc),
    Tag(Tag),
}

impl Cli {
    pub fn run(self) -> ExitCode {
        if let Err(e) = self.try_run() {
            eprintln!("{}", e);
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        }
    }

    fn try_run(self) -> Result<(), Error> {
        let wallet_service = WalletServiceImpl::new();
        let tag_service = TagServiceImpl::new();
        let rpc_service = RpcServiceImpl::new();
        let transaction_service = TransactionServiceImpl::new();
        let cipher_service = CipherServiceImpl::new();
        let wallet_repository = ConfyClient::<WalletConfy>::new();

        match self.command {
            Commands::Balance(balance) => balance.execute(transaction_service, wallet_repository),
            Commands::Create(create) => create.execute(wallet_service, wallet_repository),
            Commands::Edit(edit) => edit.execute(wallet_service, wallet_repository),
            Commands::Faucet(faucet) => faucet.execute(transaction_service, wallet_repository),
            Commands::Import(import) => import.execute(wallet_service, wallet_repository),
            Commands::List(list) => list.execute(wallet_service, wallet_repository),
            Commands::NewCipher(new_cipher) => {
                new_cipher.execute(cipher_service, wallet_repository)
            }
            Commands::Rpc(rpc) => rpc.execute(rpc_service, wallet_repository),
            Commands::Tag(tag) => tag.execute(tag_service, wallet_repository),
            Commands::Export(export) => export.execute(wallet_service, wallet_repository),
        }
    }
}
