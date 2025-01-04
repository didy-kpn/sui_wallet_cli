use crate::{
    commands::{TagService, WalletRepository},
    error::Result,
    models::{tag_list::TagList, wallet_confy::WalletConfy},
    views::tag_view::TagListView,
};

#[derive(Default)]
pub struct TagServiceImpl;

pub struct CreateTags {
    pub names: TagList,
}

pub struct RemoveTags {
    pub names: TagList,
}

pub struct ListTags {
    pub json: bool,
}

impl TagServiceImpl {
    pub fn new() -> Self {
        Self
    }
}

impl<R: WalletRepository<WalletConfy>> TagService<R> for TagServiceImpl {
    fn create(&self, create_tags: CreateTags, repository: R) -> Result<()> {
        let mut wallet_confy = repository.load()?;

        wallet_confy.mut_tags().extend(&create_tags.names);

        repository.store(wallet_confy.clone())?;

        println!("Tags added successfully");
        Ok(())
    }

    fn remote(&self, remove_tags: RemoveTags, repository: R) -> Result<()> {
        let mut wallet_confy = repository.load()?;

        for wallet in wallet_confy.mut_wallets().values_mut() {
            wallet.mut_tags().remove(&remove_tags.names);
        }

        wallet_confy.mut_tags().remove(&remove_tags.names);

        repository.store(wallet_confy.clone())?;

        println!("Tags removed successfully");

        Ok(())
    }

    fn list(&self, list_tags: ListTags, repository: R) -> Result<()> {
        let tag_view = TagListView::from_tag_list(repository.load()?.get_tags());

        if list_tags.json {
            println!("{}", tag_view.to_json_string());
        } else {
            tag_view.to_table().printstd();
        }

        Ok(())
    }
}
