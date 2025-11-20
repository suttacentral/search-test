use crate::identifiers::{DictionaryUrl, TextUrl};
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Hit {
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
pub struct MixedHits {
    pub hits: Vec<Hit>,
}

impl Hit {
    pub fn text_url(&self) -> Option<TextUrl> {
        if let Hit::Text { url, .. } = self {
            Some(url.clone())
        } else {
            None
        }
    }

    pub fn dictionary_url(&self) -> Option<DictionaryUrl> {
        if let Hit::Dictionary { url, .. } = self {
            Some(url.clone())
        } else {
            None
        }
    }
}
