use anyhow::Result;
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

#[derive(Clone, Debug, PartialEq)]
pub enum SearchResultKey {
    Text { url: TextUrl },
    Dictionary { url: DictionaryUrl },
    Suttaplex { uid: SuttaplexUid },
}

impl SearchResultKey {
    fn from_one_of(
        suttaplex: Option<SuttaplexUid>,
        sutta: Option<TextUrl>,
        dictionary: Option<DictionaryUrl>,
    ) -> Result<Option<SearchResultKey>> {
        if let Some(uid) = suttaplex {
            return Ok(Some(SearchResultKey::Suttaplex { uid: uid.clone() }));
        };
        if let Some(url) = sutta {
            return Ok(Some(SearchResultKey::Text { url: url.clone() }));
        };
        if let Some(url) = dictionary {
            return Ok(Some(SearchResultKey::Dictionary { url: url.clone() }));
        };
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_key_from_suttaplex() {
        let suttaplex = Some(SuttaplexUid::from("mn1"));
        let key = SearchResultKey::from_one_of(suttaplex, None, None)
            .unwrap()
            .unwrap();
        assert_eq!(
            key,
            SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn1")
            }
        );
    }

    #[test]
    fn get_key_from_sutta() {
        let sutta = Some(TextUrl::from("mn1"));
        let key = SearchResultKey::from_one_of(None, sutta, None)
            .unwrap()
            .unwrap();
        assert_eq!(
            key,
            SearchResultKey::Text {
                url: TextUrl::from("mn1")
            }
        );
    }
}
