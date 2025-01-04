use super::tag::Tag;
use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, str::FromStr};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TagList(HashSet<Tag>);

impl TagList {
    pub fn iter(&self) -> impl Iterator<Item = &Tag> {
        self.0.iter()
    }
    pub fn contains(&self, tag: &Tag) -> bool {
        self.0.contains(tag)
    }

    pub fn contains_all(&self, tags: &Self) -> bool {
        tags.iter().all(|tag| self.contains(tag))
    }

    pub fn join(&self, separator: &str) -> String {
        let mut list = self.iter().map(|tag| tag.to_string()).collect::<Vec<_>>();
        list.sort();

        list.join(separator)
    }

    pub fn extend(&mut self, tags: &Self) {
        self.0.extend(tags.iter().cloned());
    }

    pub fn remove(&mut self, tags: &Self) {
        for tag in tags.iter() {
            self.0.remove(tag);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl FromStr for TagList {
    type Err = Error;

    fn from_str(tags: &str) -> std::result::Result<Self, Self::Err> {
        let tags = tags.split(',').collect::<Vec<&str>>();
        let mut tag_list = HashSet::new();
        for tag in tags {
            tag_list.insert(Tag::new(tag)?);
        }
        Ok(Self(tag_list))
    }
}

#[cfg(test)]
mod tests {
    use super::TagList;
    use crate::models::tag::Tag;
    use std::str::FromStr;

    #[test]
    fn test_tag_list() {
        let mut list = TagList::default();
        assert!(list.is_empty());

        list.extend(&TagList::from_str("tag1,tag2,tag3,tag3").unwrap());

        assert!(!list.is_empty());
        assert_eq!(list.iter().count(), 3);
        assert!(list.contains(&Tag::new("tag1").unwrap()));
        assert!(list.contains(&Tag::new("tag2").unwrap()));
        assert!(list.contains(&Tag::new("tag3").unwrap()));
        assert!(!list.contains(&Tag::new("tag4").unwrap()));

        assert!(list.contains_all(&TagList::from_str("tag1,tag2,tag3").unwrap()));
        assert!(list.contains_all(&TagList::from_str("tag1,tag2").unwrap()));
        assert!(!list.contains_all(&TagList::from_str("tag1,tag4").unwrap()));

        assert_eq!(list.join("."), "tag1.tag2.tag3".to_string());
        assert_eq!(list.join(" "), "tag1 tag2 tag3".to_string());

        list.remove(&TagList::from_str("tag2,tag3").unwrap());

        assert_eq!(list.iter().count(), 1);
        assert!(list.contains(&Tag::new("tag1").unwrap()));
        assert!(!list.contains(&Tag::new("tag2").unwrap()));
        assert!(!list.contains(&Tag::new("tag3").unwrap()));
    }
}
