use super::{Command, TagService};
use crate::{
    error::Result,
    models::tag_list::TagList,
    services::tag_service::{CreateTags, ListTags, RemoveTags},
};
use clap::{Args, Subcommand};
use std::str::FromStr;

#[derive(Debug, Args)]
pub struct Tag {
    #[command(subcommand)]
    pub command: TagCommand,
}

#[derive(Debug, Subcommand, Clone)]
pub enum TagCommand {
    Add {
        #[arg(value_parser = TagList::from_str)]
        names: TagList,
    },
    Remove {
        #[arg(value_parser = TagList::from_str)]
        names: TagList,
    },
    List {
        #[arg(short, long)]
        json: bool,
    },
}

impl<S: TagService<R>, R> Command<S, R> for Tag {
    fn execute(&self, service: S, repository: R) -> Result<()> {
        match self.command.clone() {
            TagCommand::Add { names } => service.create(CreateTags { names }, repository),
            TagCommand::Remove { names } => service.remote(RemoveTags { names }, repository),
            TagCommand::List { json } => service.list(ListTags { json }, repository),
        }
    }
}
