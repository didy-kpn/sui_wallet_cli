use crate::models::tag_list::TagList;
use prettytable::{cell, row, Table};
use serde_json::json;

pub struct TagListView(Vec<String>);

impl TagListView {
    pub fn from_tag_list(tag_list: &TagList) -> Self {
        let mut tags = tag_list
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        tags.sort();
        Self(tags)
    }

    pub fn to_table(&self) -> Table {
        let mut table = Table::new();
        table.add_row(row!["Name"]);
        for tag in self.0.iter() {
            table.add_row(row![cell!(tag),]);
        }
        table
    }

    pub fn to_json_string(&self) -> String {
        let json = json![{
            "tags": self.0
        }];
        serde_json::to_string(&json).unwrap()
    }
}
