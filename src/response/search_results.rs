use crate::identifiers::{DictionaryUrl, SearchType, SuttaplexUid, TextUrl, VolpageReference};
use crate::response::dictionary::dictionary_results;
use crate::response::suttaplex::suttaplex_results;
use crate::response::texts::text_results;
use anyhow::Result;

#[derive(Clone, Debug, PartialEq)]
pub enum SearchResultsNewStyle {
    Text { results: Vec<TextUrl> },
    Dictionary { results: Vec<DictionaryUrl> },
    Suttaplex { results: Vec<SuttaplexUid> },
    Volpage { results: Vec<VolpageReference> },
}

impl SearchResultsNewStyle {
    pub fn new(search_type: SearchType, json: &str) -> Result<SearchResultsNewStyle> {
        match search_type {
            SearchType::Text => Ok(SearchResultsNewStyle::Text {
                results: text_results(json)?,
            }),
            SearchType::Dictionary => Ok(SearchResultsNewStyle::Dictionary {
                results: dictionary_results(json)?,
            }),
            SearchType::Suttaplex => Ok(SearchResultsNewStyle::Suttaplex {
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
            SearchResultsNewStyle::new(SearchType::Text, TEXT_JSON).unwrap(),
            SearchResultsNewStyle::Text {
                results: vec![TextUrl::from("/mn1/en/sujato")]
            }
        );
    }

    #[test]
    fn constructs_dictionary_results() {
        assert_eq!(
            SearchResultsNewStyle::new(SearchType::Dictionary, DICTIONARY_JSON).unwrap(),
            SearchResultsNewStyle::Dictionary {
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
            SearchResultsNewStyle::new(SearchType::Suttaplex, SUTTAPLEX_JSON).unwrap(),
            SearchResultsNewStyle::Suttaplex {
                results: vec![SuttaplexUid::from("mn1")]
            }
        )
    }
}
