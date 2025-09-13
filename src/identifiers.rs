use serde::Deserialize;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct TextUrl(String);

impl Display for TextUrl {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for TextUrl {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct DictionaryUrl(String);

impl Display for DictionaryUrl {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for DictionaryUrl {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct SuttaplexUid(String);

impl Display for SuttaplexUid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for SuttaplexUid {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

pub enum SearchResult {
    Text { url: TextUrl },
    Dictionary { url: DictionaryUrl },
    Suttaplex { uid: SuttaplexUid },
}
