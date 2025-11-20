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

impl Hit {
    pub fn text_url(&self) -> Option<TextUrl> {
        if let Hit::Text { url, .. } = self {
            Some(url.clone())
        } else {
            None
        }
    }
}
