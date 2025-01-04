pub mod cli;
pub mod commands;
pub mod error;
pub mod models;
pub mod services;
pub mod storages;
pub mod views;

use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    dotenvy::dotenv().ok();
    cli::Cli::parse().run()
}
