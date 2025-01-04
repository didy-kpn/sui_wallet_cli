use crate::{commands::WalletRepository, error::Result};
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;

const APP_NAME: &str = "sui_wallet_cli";
const CONFIG_FILE: Option<&str> = Some("wallets");

pub struct ConfyClient<C>(PhantomData<C>);

impl<C> ConfyClient<C> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<C> Default for ConfyClient<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: Default + DeserializeOwned + Serialize> WalletRepository<C> for ConfyClient<C> {
    fn load(&self) -> Result<C> {
        Ok(confy::load(APP_NAME, CONFIG_FILE)?)
    }

    fn store(&self, confy: C) -> Result<()> {
        Ok(confy::store(APP_NAME, CONFIG_FILE, confy)?)
    }
}
