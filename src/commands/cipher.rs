use super::{CipherService, Command};
use crate::error::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct Cipher {}

impl<S: CipherService, R> Command<S, R> for Cipher {
    fn execute(&self, service: S, _repository: R) -> Result<()> {
        service.create()
    }
}
