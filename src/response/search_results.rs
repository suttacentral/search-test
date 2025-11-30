use crate::identifiers::{DictionaryUrl, SearchType, SuttaplexUid, TextUrl, VolpageReference};
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
    pub fn new(search_type: SearchType, json: &str) -> Result<SearchResults> {
        match search_type {
            SearchType::Text => Ok(SearchResults::Text {
                results: text_results(json)?,
            }),
            SearchType::Dictionary => Ok(SearchResults::Dictionary {
                results: dictionary_results(json)?,
            }),
            SearchType::Suttaplex => Ok(SearchResults::Suttaplex {
                results: suttaplex_results(json)?,
            }),
            SearchType::Volpage => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXT_JSON: &str = r#"
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

    const DICTIONARY_JSON: &str = r#"
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

    const SUTTAPLEX_JSON: &str = r#"
    {
        "suttaplex": [
            {
                "uid": "mn1"
            }
        ]
    }
    "#;

    #[test]
    fn constructs_text_results() {
        assert_eq!(
            SearchResults::new(SearchType::Text, TEXT_JSON).unwrap(),
            SearchResults::Text {
                results: vec![TextUrl::from("/mn1/en/sujato")]
            }
        );
    }

    #[test]
    fn constructs_dictionary_results() {
        assert_eq!(
            SearchResults::new(SearchType::Dictionary, DICTIONARY_JSON).unwrap(),
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
        assert_eq!(
            SearchResults::new(SearchType::Suttaplex, SUTTAPLEX_JSON).unwrap(),
            SearchResults::Suttaplex {
                results: vec![SuttaplexUid::from("mn1")]
            }
        )
    }
}
