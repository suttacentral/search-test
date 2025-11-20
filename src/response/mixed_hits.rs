use crate::identifiers::{DictionaryUrl, TextUrl};
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum MixedHit {
    Dictionary {
        category: String,
        url: DictionaryUrl,
    },
    Text {
        uid: String,
        lang: String,
        author_uid: Option<String>,
        url: TextUrl,
    },
}

#[derive(Deserialize, Debug)]
pub struct TopLevel {
    pub hits: Vec<MixedHit>,
}

impl MixedHit {
    pub fn text_url(&self) -> Option<TextUrl> {
        if let MixedHit::Text { url, .. } = self {
            Some(url.clone())
        } else {
            None
        }
    }

    pub fn dictionary_url(&self) -> Option<DictionaryUrl> {
        if let MixedHit::Dictionary { url, .. } = self {
            Some(url.clone())
        } else {
            None
        }
    }
}
