use crate::identifiers::{DictionaryUrl, SearchResultKey, SuttaplexUid, TextUrl, VolpageReference};
use crate::response::dictionary::dictionary_results;
use crate::response::suttaplex::suttaplex_results;
use crate::response::texts::text_results;
use crate::response::volpage::volpage_results;
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
            SearchResultKey::Volpage { reference } => Ok(SearchResults::Volpage {
                expected: reference.clone(),
                results: volpage_results(json)?,
            }),
        }
    }

    pub fn found(&self) -> bool {
        match self {
            Self::Text { expected, results } => results.contains(expected),
            Self::Suttaplex { expected, results } => results.contains(expected),
            Self::Dictionary { expected, results } => results.contains(expected),
            Self::Volpage { expected, results } => results.contains(expected),
        }
    }

    pub fn rank(&self) -> Option<usize> {
        match self {
            Self::Text { expected, results } => Self::rank_in_results(expected, results),
            Self::Dictionary { expected, results } => Self::rank_in_results(expected, results),
            Self::Suttaplex { expected, results } => Self::rank_in_results(expected, results),
            Self::Volpage { expected, results } => Self::rank_in_results(expected, results),
        }
    }

    fn rank_in_results<T: PartialEq>(item: &T, results: &[T]) -> Option<usize> {
        results
            .iter()
            .position(|hit| hit == item)
            .map(|position| position + 1)
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

    const VOLPAGE_JSON: &str = r#"
        {
            "hits": [
                {
                    "volpage": "PTS SN ii 1"
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

    #[test]
    fn new_volpage_results() {
        let key = SearchResultKey::Volpage {
            reference: VolpageReference::from("PTS SN ii 1"),
        };

        assert_eq!(
            SearchResults::new(&key, VOLPAGE_JSON).unwrap(),
            SearchResults::Volpage {
                expected: VolpageReference::from("PTS SN ii 1"),
                results: vec![VolpageReference::from("PTS SN ii 1")]
            }
        )
    }

    #[test]
    fn text_is_found() {
        let results = SearchResults::Text {
            expected: TextUrl::from("/mn1/en/bodhi"),
            results: vec![TextUrl::from("/mn1/en/bodhi")],
        };

        assert!(results.found());
    }

    #[test]
    fn text_is_missing() {
        let results = SearchResults::Text {
            expected: TextUrl::from("/mn1/en/bodhi"),
            results: vec![],
        };

        assert!(!results.found());
    }

    #[test]
    fn dictionary_is_found() {
        let results = SearchResults::Dictionary {
            expected: DictionaryUrl::from("/define/metta"),
            results: vec![DictionaryUrl::from("/define/metta")],
        };

        assert!(results.found());
    }

    #[test]
    fn dictionary_is_missing() {
        let results = SearchResults::Dictionary {
            expected: DictionaryUrl::from("/define/metta"),
            results: vec![DictionaryUrl::from("/define/dosa")],
        };

        assert!(!results.found());
    }

    #[test]
    fn suttaplex_is_found() {
        let results = SearchResults::Suttaplex {
            expected: SuttaplexUid::from("mn1"),
            results: vec![SuttaplexUid::from("mn1"), SuttaplexUid::from("mn2")],
        };

        assert!(results.found());
    }

    #[test]
    fn suttaplex_is_not_found() {
        let results = SearchResults::Suttaplex {
            expected: SuttaplexUid::from("mn1"),
            results: vec![SuttaplexUid::from("mn2"), SuttaplexUid::from("mn3")],
        };

        assert!(!results.found());
    }

    #[test]
    fn suttaplex_has_rank() {
        let search = SearchResults::Suttaplex {
            expected: SuttaplexUid::from("mn3"),
            results: vec![SuttaplexUid::from("mn2"), SuttaplexUid::from("mn3")],
        };

        assert_eq!(search.rank(), Some(2));
    }

    #[test]
    fn missing_suttaplex_has_no_rank() {
        let results = SearchResults::Suttaplex {
            expected: SuttaplexUid::from("mn3"),
            results: Vec::new(),
        };

        assert_eq!(results.rank(), None);
    }

    #[test]
    fn text_has_rank() {
        let results = SearchResults::Text {
            expected: TextUrl::from("/mn1/en/bodhi"),
            results: vec![
                TextUrl::from("/mn1/en/bodhi"),
                TextUrl::from("/mn1/en/sujato"),
            ],
        };

        assert_eq!(results.rank(), Some(1));
    }

    #[test]
    fn text_has_no_rank() {
        let results = SearchResults::Text {
            expected: TextUrl::from("/mn1/en/bodhi"),
            results: Vec::new(),
        };

        assert_eq!(results.rank(), None);
    }

    #[test]
    fn dictionary_has_rank() {
        let results = SearchResults::Dictionary {
            expected: DictionaryUrl::from("/define/metta"),
            results: vec![DictionaryUrl::from("/define/metta")],
        };
        assert_eq!(results.rank(), Some(1))
    }

    #[test]
    fn dictionary_has_no_rank() {
        let results = SearchResults::Dictionary {
            expected: DictionaryUrl::from("/define/metta"),
            results: Vec::new(),
        };
        assert_eq!(results.rank(), None)
    }
}
