use crate::identifiers::SearchType::{Dictionary, Suttaplex, Text, Volpage};
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

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct VolpageReference(String);

impl From<&str> for VolpageReference {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SearchResultKey {
    Text { url: TextUrl },
    Dictionary { url: DictionaryUrl },
    Suttaplex { uid: SuttaplexUid },
    Volpage { reference: VolpageReference },
}

#[derive(Clone, Debug, PartialEq)]
pub enum SearchType {
    Text,
    Dictionary,
    Suttaplex,
    Volpage,
}

impl From<&SearchResultKey> for SearchType {
    fn from(key: &SearchResultKey) -> Self {
        match key {
            SearchResultKey::Text { .. } => Text,
            SearchResultKey::Dictionary { .. } => Dictionary,
            SearchResultKey::Suttaplex { .. } => Suttaplex,
            SearchResultKey::Volpage { .. } => Volpage,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_type_from_text_key() {
        let text_key = SearchResultKey::Text {
            url: TextUrl::from("/mn1/en/sujato"),
        };
        assert_eq!(SearchType::from(&text_key), SearchType::Text);
    }

    #[test]
    fn search_type_from_dictionary_key() {
        let dictionary_key = SearchResultKey::Dictionary {
            url: DictionaryUrl::from("/define/metta"),
        };
        assert_eq!(SearchType::from(&dictionary_key), SearchType::Dictionary);
    }

    #[test]
    fn search_type_from_suttaplex_key() {
        let suttaplex_key = SearchResultKey::Suttaplex {
            uid: SuttaplexUid::from("mn1"),
        };
        assert_eq!(SearchType::from(&suttaplex_key), SearchType::Suttaplex);
    }

    #[test]
    fn search_type_from_volpage_key() {
        let volpage_key = SearchResultKey::Volpage {
            reference: VolpageReference::from("PTS SN ii 1"),
        };
        assert_eq!(SearchType::from(&volpage_key), SearchType::Volpage);
    }
}
