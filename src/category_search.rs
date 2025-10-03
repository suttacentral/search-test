use crate::identifiers::{DictionaryUrl, SearchResultKey, SuttaplexUid, TextUrl};
use crate::response::SearchResults;
use crate::test_result::Outcome;

#[derive(Debug, PartialEq)]
pub enum CategorySearch {
    Text {
        search_for: TextUrl,
        in_sequence: Vec<TextUrl>,
    },
    Dictionary {
        search_for: DictionaryUrl,
        in_sequence: Vec<DictionaryUrl>,
    },
    Suttaplex {
        search_for: SuttaplexUid,
        in_sequence: Vec<SuttaplexUid>,
    },
}

impl CategorySearch {
    pub fn new(key: &SearchResultKey, results: &SearchResults) -> Self {
        match key {
            SearchResultKey::Suttaplex { uid } => Self::Suttaplex {
                search_for: uid.clone(),
                in_sequence: results.suttaplex.to_vec(),
            },
            SearchResultKey::Dictionary { url } => Self::Dictionary {
                search_for: url.clone(),
                in_sequence: results.dictionary.to_vec(),
            },
            SearchResultKey::Text { url } => Self::Text {
                search_for: url.clone(),
                in_sequence: results.text.to_vec(),
            },
        }
    }

    pub fn found(&self) -> bool {
        match self {
            Self::Text {
                search_for,
                in_sequence,
            } => in_sequence.contains(search_for),
            Self::Suttaplex {
                search_for,
                in_sequence,
            } => in_sequence.contains(search_for),
            Self::Dictionary {
                search_for,
                in_sequence,
            } => in_sequence.contains(search_for),
        }
    }

    fn rank(&self) -> Option<usize> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identifiers::{SearchResultKey, SuttaplexUid};
    use crate::response::SearchResults;

    #[test]
    fn text_in_results() {
        let key = SearchResultKey::Text {
            url: TextUrl::from("/mn1/en/bodhi"),
        };

        let search_results = SearchResults {
            text: vec![TextUrl::from("/mn1/en/bodhi")],
            dictionary: Vec::new(),
            suttaplex: Vec::new(),
        };

        let search = CategorySearch::new(&key, &search_results);

        assert_eq!(
            search,
            CategorySearch::Text {
                search_for: TextUrl::from("/mn1/en/bodhi"),
                in_sequence: vec![TextUrl::from("/mn1/en/bodhi")]
            }
        )
    }

    #[test]
    fn dictionary_in_results() {
        let key = SearchResultKey::Dictionary {
            url: DictionaryUrl::from("/define/metta"),
        };

        let search_results = SearchResults {
            text: Vec::new(),
            dictionary: vec![DictionaryUrl::from("/define/metta")],
            suttaplex: Vec::new(),
        };

        let search = CategorySearch::new(&key, &search_results);

        assert_eq!(
            search,
            CategorySearch::Dictionary {
                search_for: DictionaryUrl::from("/define/metta"),
                in_sequence: vec![DictionaryUrl::from("/define/metta")]
            }
        )
    }

    #[test]
    fn create_suttaplex_search_result() {
        let key = SearchResultKey::Suttaplex {
            uid: SuttaplexUid::from("mn1"),
        };

        let search_results = SearchResults {
            text: Vec::new(),
            dictionary: Vec::new(),
            suttaplex: vec![SuttaplexUid::from("mn1")],
        };

        let search = CategorySearch::new(&key, &search_results);

        assert_eq!(
            search,
            CategorySearch::Suttaplex {
                search_for: SuttaplexUid::from("mn1"),
                in_sequence: vec![SuttaplexUid::from("mn1")]
            }
        )
    }

    #[test]
    fn text_is_found() {
        let search = CategorySearch::Text {
            search_for: TextUrl::from("/mn1/en/bodhi"),
            in_sequence: vec![TextUrl::from("/mn1/en/bodhi")],
        };

        assert!(search.found());
    }

    #[test]
    fn text_is_missing() {
        let search = CategorySearch::Text {
            search_for: TextUrl::from("/mn1/en/bodhi"),
            in_sequence: vec![],
        };

        assert!(!search.found());
    }

    #[test]
    fn dictionary_is_found() {
        let search = CategorySearch::Dictionary {
            search_for: DictionaryUrl::from("/define/metta"),
            in_sequence: vec![DictionaryUrl::from("/define/metta")],
        };

        assert!(search.found());
    }

    #[test]
    fn dictionary_is_missing() {
        let search = CategorySearch::Dictionary {
            search_for: DictionaryUrl::from("/define/metta"),
            in_sequence: vec![DictionaryUrl::from("/define/dosa")],
        };

        assert!(!search.found());
    }
}
