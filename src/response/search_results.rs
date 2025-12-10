use crate::identifiers::{DictionaryUrl, SearchResultKey, SuttaplexUid, TextUrl, VolpageReference};
use crate::response::dictionary::dictionary_results;
use crate::response::suttaplex::suttaplex_results;
use crate::response::texts::text_results;
use anyhow::Result;

#[derive(Clone, Debug, PartialEq)]
pub enum SearchResults {
    Text {
        expected: TextUrl,
        results: Vec<TextUrl>,
    },
    Dictionary {
        expected: DictionaryUrl,
        results: Vec<DictionaryUrl>,
    },
    Suttaplex {
        expected: SuttaplexUid,
        results: Vec<SuttaplexUid>,
    },
    Volpage {
        expected: VolpageReference,
        results: Vec<VolpageReference>,
    },
}

impl SearchResults {
    pub fn new(key: &SearchResultKey, json: &str) -> Result<SearchResults> {
        match key {
            SearchResultKey::Text { url } => Ok(SearchResults::Text {
                expected: url.clone(),
                results: text_results(json)?,
            }),
            SearchResultKey::Dictionary { url } => Ok(SearchResults::Dictionary {
                expected: url.clone(),
                results: dictionary_results(json)?,
            }),
            SearchResultKey::Suttaplex { uid } => Ok(SearchResults::Suttaplex {
                expected: uid.clone(),
                results: suttaplex_results(json)?,
            }),
            SearchResultKey::Volpage { reference } => todo!(),
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
    fn new_text_results() {
        let key = SearchResultKey::Text {
            url: TextUrl::from("/mn1/en/sujato"),
        };

        assert_eq!(
            SearchResults::new(&key, TEXT_JSON).unwrap(),
            SearchResults::Text {
                expected: TextUrl::from("/mn1/en/sujato"),
                results: vec![TextUrl::from("/mn1/en/sujato")]
            }
        );
    }

    #[test]
    fn new_dictionary_results() {
        let key = SearchResultKey::Dictionary {
            url: DictionaryUrl::from("/define/metta"),
        };

        assert_eq!(
            SearchResults::new(&key, DICTIONARY_JSON).unwrap(),
            SearchResults::Dictionary {
                expected: DictionaryUrl::from("/define/metta"),
                results: vec![
                    DictionaryUrl::from("/define/metta"),
                    DictionaryUrl::from("/define/dosa")
                ]
            }
        )
    }

    #[test]
    fn new_suttaplex_results() {
        let key = SearchResultKey::Suttaplex {
            uid: SuttaplexUid::from("mn1"),
        };

        assert_eq!(
            SearchResults::new(&key, SUTTAPLEX_JSON).unwrap(),
            SearchResults::Suttaplex {
                expected: SuttaplexUid::from("mn1"),
                results: vec![SuttaplexUid::from("mn1")]
            }
        )
    }
}
