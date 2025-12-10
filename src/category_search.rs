use crate::identifiers::{DictionaryUrl, SearchResultKey, SuttaplexUid, TextUrl};
use crate::response::search_results::SearchResults;

#[derive(Clone, Debug, PartialEq)]
pub enum CategorySearch {
    Text {
        expected: TextUrl,
        in_results: Vec<TextUrl>,
    },
    Dictionary {
        expected: DictionaryUrl,
        in_results: Vec<DictionaryUrl>,
    },
    Suttaplex {
        expected: SuttaplexUid,
        in_results: Vec<SuttaplexUid>,
    },
}

impl CategorySearch {
    pub fn new(key: &SearchResultKey, results: &SearchResults) -> Self {
        match results {
            SearchResults::Text { expected, results } => CategorySearch::Text {
                expected: expected.clone(),
                in_results: results.clone(),
            },
            SearchResults::Dictionary { expected, results } => CategorySearch::Dictionary {
                expected: expected.clone(),
                in_results: results.clone(),
            },
            SearchResults::Suttaplex { expected, results } => CategorySearch::Suttaplex {
                expected: expected.clone(),
                in_results: results.clone(),
            },
            SearchResults::Volpage { expected, results } => todo!(),
        }
    }

    fn rank_in_results<T: PartialEq>(item: &T, results: &[T]) -> Option<usize> {
        results
            .iter()
            .position(|hit| hit == item)
            .map(|position| position + 1)
    }

    pub fn found(&self) -> bool {
        match self {
            Self::Text {
                expected,
                in_results,
            } => in_results.contains(expected),
            Self::Suttaplex {
                expected,
                in_results,
            } => in_results.contains(expected),
            Self::Dictionary {
                expected,
                in_results,
            } => in_results.contains(expected),
        }
    }

    pub fn rank(&self) -> Option<usize> {
        match self {
            Self::Text {
                expected,
                in_results,
            } => Self::rank_in_results(expected, in_results),
            Self::Dictionary {
                expected,
                in_results,
            } => Self::rank_in_results(expected, in_results),
            Self::Suttaplex {
                expected,
                in_results,
            } => Self::rank_in_results(expected, in_results),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_text() {
        let key = SearchResultKey::Text {
            url: TextUrl::from("/mn1/en/bodhi"),
        };

        let search_results = SearchResults::Text {
            expected: TextUrl::from("/mn1/en/bodhi"),
            results: vec![TextUrl::from("/mn1/en/bodhi")],
        };

        assert_eq!(
            CategorySearch::new(&key, &search_results),
            CategorySearch::Text {
                expected: TextUrl::from("/mn1/en/bodhi"),
                in_results: vec![TextUrl::from("/mn1/en/bodhi")]
            }
        );
    }

    #[test]
    fn new_dictionary() {
        let key = SearchResultKey::Dictionary {
            url: DictionaryUrl::from("/define/metta"),
        };

        let search_results = SearchResults::Dictionary {
            expected: DictionaryUrl::from("/define/metta"),
            results: vec![DictionaryUrl::from("/define/metta")],
        };

        assert_eq!(
            CategorySearch::new(&key, &search_results),
            CategorySearch::Dictionary {
                expected: DictionaryUrl::from("/define/metta"),
                in_results: vec![DictionaryUrl::from("/define/metta")]
            }
        );
    }

    #[test]
    fn new_suttaplex() {
        let key = SearchResultKey::Suttaplex {
            uid: SuttaplexUid::from("mn1"),
        };

        let search_results = SearchResults::Suttaplex {
            expected: SuttaplexUid::from("mn1"),
            results: vec![SuttaplexUid::from("mn1")],
        };

        assert_eq!(
            CategorySearch::new(&key, &search_results),
            CategorySearch::Suttaplex {
                expected: SuttaplexUid::from("mn1"),
                in_results: vec![SuttaplexUid::from("mn1")]
            }
        );
    }

    #[test]
    fn text_is_found() {
        let search = CategorySearch::Text {
            expected: TextUrl::from("/mn1/en/bodhi"),
            in_results: vec![TextUrl::from("/mn1/en/bodhi")],
        };

        assert!(search.found());
    }

    #[test]
    fn text_is_missing() {
        let search = CategorySearch::Text {
            expected: TextUrl::from("/mn1/en/bodhi"),
            in_results: vec![],
        };

        assert!(!search.found());
    }

    #[test]
    fn dictionary_is_found() {
        let search = CategorySearch::Dictionary {
            expected: DictionaryUrl::from("/define/metta"),
            in_results: vec![DictionaryUrl::from("/define/metta")],
        };

        assert!(search.found());
    }

    #[test]
    fn dictionary_is_missing() {
        let search = CategorySearch::Dictionary {
            expected: DictionaryUrl::from("/define/metta"),
            in_results: vec![DictionaryUrl::from("/define/dosa")],
        };

        assert!(!search.found());
    }

    #[test]
    fn suttaplex_is_found() {
        let search = CategorySearch::Suttaplex {
            expected: SuttaplexUid::from("mn1"),
            in_results: vec![SuttaplexUid::from("mn1"), SuttaplexUid::from("mn2")],
        };

        assert!(search.found());
    }

    #[test]
    fn suttaplex_is_not_found() {
        let search = CategorySearch::Suttaplex {
            expected: SuttaplexUid::from("mn1"),
            in_results: vec![SuttaplexUid::from("mn2"), SuttaplexUid::from("mn3")],
        };

        assert!(!search.found());
    }

    #[test]
    fn suttaplex_has_rank() {
        let search = CategorySearch::Suttaplex {
            expected: SuttaplexUid::from("mn3"),
            in_results: vec![SuttaplexUid::from("mn2"), SuttaplexUid::from("mn3")],
        };

        assert_eq!(search.rank(), Some(2));
    }

    #[test]
    fn missing_suttaplex_has_no_rank() {
        let search = CategorySearch::Suttaplex {
            expected: SuttaplexUid::from("mn3"),
            in_results: Vec::new(),
        };

        assert_eq!(search.rank(), None);
    }

    #[test]
    fn text_has_rank() {
        let search = CategorySearch::Text {
            expected: TextUrl::from("/mn1/en/bodhi"),
            in_results: vec![
                TextUrl::from("/mn1/en/bodhi"),
                TextUrl::from("/mn1/en/sujato"),
            ],
        };

        assert_eq!(search.rank(), Some(1));
    }

    #[test]
    fn text_has_no_rank() {
        let search = CategorySearch::Text {
            expected: TextUrl::from("/mn1/en/bodhi"),
            in_results: Vec::new(),
        };

        assert_eq!(search.rank(), None);
    }

    #[test]
    fn dictionary_has_rank() {
        let search = CategorySearch::Dictionary {
            expected: DictionaryUrl::from("/define/metta"),
            in_results: vec![DictionaryUrl::from("/define/metta")],
        };
        assert_eq!(search.rank(), Some(1))
    }

    #[test]
    fn dictionary_has_no_rank() {
        let search = CategorySearch::Dictionary {
            expected: DictionaryUrl::from("/define/metta"),
            in_results: Vec::new(),
        };
        assert_eq!(search.rank(), None)
    }
}
