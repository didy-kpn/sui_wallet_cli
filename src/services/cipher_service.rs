use crate::{commands::CipherService, error::Result, models::cipher::Cipher};

#[derive(Default)]
pub struct CipherServiceImpl;

impl CipherServiceImpl {
    pub fn new() -> Self {
        Self
    }
}

impl CipherService for CipherServiceImpl {
    fn create(&self) -> Result<()> {
        println!("{}", Cipher::default());
        Ok(())
    }
}
