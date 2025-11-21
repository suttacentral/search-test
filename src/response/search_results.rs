use crate::identifiers::{DictionaryUrl, SearchResultKey, SuttaplexUid, TextUrl, VolpageReference};
use crate::response::dictionary::dictionary_results;
use crate::response::suttaplex::suttaplex_results;
use crate::response::texts::text_results;
use anyhow::Result;

#[derive(Clone, Debug, PartialEq)]
pub enum SearchResults {
    Text { results: Vec<TextUrl> },
    Dictionary { results: Vec<DictionaryUrl> },
    Suttaplex { results: Vec<SuttaplexUid> },
    Volpage { results: Vec<VolpageReference> },
}

impl SearchResults {
    pub fn new(key: SearchResultKey, json: &str) -> Result<SearchResults> {
        match key {
            SearchResultKey::Text { .. } => Ok(SearchResults::Text {
                results: text_results(json)?,
            }),
            SearchResultKey::Dictionary { .. } => Ok(SearchResults::Dictionary {
                results: dictionary_results(json)?,
            }),
            SearchResultKey::Suttaplex { .. } => Ok(SearchResults::Suttaplex {
                results: suttaplex_results(json)?,
            }),
            SearchResultKey::Volpage { .. } => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_text_results() {
        let key = SearchResultKey::Text {
            url: TextUrl::from("/mn1/en/sujato"),
        };

        let json = r#"
        {
            "hits": [
                {
                    "uid": "mn1",
                    "lang": "en",
                    "author_uid": "sujato",
                    "url": "/mn1/en/sujato"
                }
            ]
        }
        "#;

        assert_eq!(
            SearchResults::new(key, json).unwrap(),
            SearchResults::Text {
                results: vec![TextUrl::from("/mn1/en/sujato")]
            }
        );
    }

    #[test]
    fn constructs_dictionary_results() {
        let key = SearchResultKey::Dictionary {
            url: DictionaryUrl::from("/define/metta"),
        };

        let json = r#"
        {
            "hits" : [
            {
                    "url": "/define/metta",
                    "category": "dictionary"
                }
            ],
            "fuzzy_dictionary": [
                {
                    "url": "/define/dosa",
                    "category": "dictionary"
                }
            ]
        }
        "#;

        assert_eq!(
            SearchResults::new(key, json).unwrap(),
            SearchResults::Dictionary {
                results: vec![
                    DictionaryUrl::from("/define/metta"),
                    DictionaryUrl::from("/define/dosa")
                ]
            }
        )
    }

    #[test]
    fn constructs_suttaplex_results() {
        let key = SearchResultKey::Suttaplex {
            uid: SuttaplexUid::from("mn1"),
        };

        let json = r#"
        {
            "suttaplex": [
                {
                    "uid": "mn1"
                }
            ]
        }
        "#;

        assert_eq!(
            SearchResults::new(key, json).unwrap(),
            SearchResults::Suttaplex {
                results: vec![SuttaplexUid::from("mn1")]
            }
        )
    }
}
