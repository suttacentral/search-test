use crate::identifiers::{DictionaryUrl, SearchResultKey, SuttaplexUid, TextUrl};
use crate::response::search_results::SearchResults;

#[derive(Clone, Debug, PartialEq)]
pub enum CategorySearch {
    Text {
        search_for: TextUrl,
        in_results: Vec<TextUrl>,
    },
    Dictionary {
        search_for: DictionaryUrl,
        in_results: Vec<DictionaryUrl>,
    },
    Suttaplex {
        search_for: SuttaplexUid,
        in_results: Vec<SuttaplexUid>,
    },
}

impl CategorySearch {
    pub fn new(key: &SearchResultKey, results: &SearchResults) -> Self {
        match results {
            SearchResults::Text { results } => match key {
                SearchResultKey::Text { url } => CategorySearch::Text {
                    search_for: url.clone(),
                    in_results: results.clone(),
                },
                _ => panic!("Mismatched key and results"),
            },
            SearchResults::Dictionary { results } => match key {
                SearchResultKey::Dictionary { url } => CategorySearch::Dictionary {
                    search_for: url.clone(),
                    in_results: results.clone(),
                },
                _ => panic!("Mismatched key and results"),
            },
            SearchResults::Suttaplex { results } => match key {
                SearchResultKey::Suttaplex { uid } => CategorySearch::Suttaplex {
                    search_for: uid.clone(),
                    in_results: results.clone(),
                },
                _ => panic!("Mismatched key and results"),
            },
            SearchResults::Volpage { results } => todo!(),
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
                search_for,
                in_results,
            } => in_results.contains(search_for),
            Self::Suttaplex {
                search_for,
                in_results,
            } => in_results.contains(search_for),
            Self::Dictionary {
                search_for,
                in_results,
            } => in_results.contains(search_for),
        }
    }

    pub fn rank(&self) -> Option<usize> {
        match self {
            Self::Text {
                search_for,
                in_results,
            } => Self::rank_in_results(search_for, in_results),
            Self::Dictionary {
                search_for,
                in_results,
            } => Self::rank_in_results(search_for, in_results),
            Self::Suttaplex {
                search_for,
                in_results,
            } => Self::rank_in_results(search_for, in_results),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identifiers::{SearchResultKey, SuttaplexUid};
    use crate::response::search_results::SearchResults;

    #[test]
    fn new_text_from_new_style_results() {
        let key = SearchResultKey::Text {
            url: TextUrl::from("/mn1/en/bodhi"),
        };

        let search_results = SearchResults::Text {
            results: vec![TextUrl::from("/mn1/en/bodhi")],
        };

        assert_eq!(
            CategorySearch::new(&key, &search_results),
            CategorySearch::Text {
                search_for: TextUrl::from("/mn1/en/bodhi"),
                in_results: vec![TextUrl::from("/mn1/en/bodhi")]
            }
        );
    }

    #[test]
    fn new_dictionary_from_new_style_results() {
        let key = SearchResultKey::Dictionary {
            url: DictionaryUrl::from("/define/metta"),
        };

        let search_results = SearchResults::Dictionary {
            results: vec![DictionaryUrl::from("/define/metta")],
        };

        assert_eq!(
            CategorySearch::new(&key, &search_results),
            CategorySearch::Dictionary {
                search_for: DictionaryUrl::from("/define/metta"),
                in_results: vec![DictionaryUrl::from("/define/metta")]
            }
        );
    }

    #[test]
    fn new_suttaplex_from_new_style_results() {
        let key = SearchResultKey::Suttaplex {
            uid: SuttaplexUid::from("mn1"),
        };

        let search_results = SearchResults::Suttaplex {
            results: vec![SuttaplexUid::from("mn1")],
        };

        assert_eq!(
            CategorySearch::new(&key, &search_results),
            CategorySearch::Suttaplex {
                search_for: SuttaplexUid::from("mn1"),
                in_results: vec![SuttaplexUid::from("mn1")]
            }
        );
    }

    #[test]
    fn text_in_results() {
        let key = SearchResultKey::Text {
            url: TextUrl::from("/mn1/en/sujato"),
        };

        let search_results = SearchResults::Text {
            results: vec![TextUrl::from("/mn1/en/sujato")],
        };

        let search = CategorySearch::new(&key, &search_results);

        assert_eq!(
            search,
            CategorySearch::Text {
                search_for: TextUrl::from("/mn1/en/sujato"),
                in_results: vec![TextUrl::from("/mn1/en/sujato")]
            }
        )
    }

    #[test]
    fn dictionary_in_results() {
        let key = SearchResultKey::Dictionary {
            url: DictionaryUrl::from("/define/metta"),
        };

        let search_results = SearchResults::Dictionary {
            results: vec![DictionaryUrl::from("/define/metta")],
        };

        let search = CategorySearch::new(&key, &search_results);

        assert_eq!(
            search,
            CategorySearch::Dictionary {
                search_for: DictionaryUrl::from("/define/metta"),
                in_results: vec![DictionaryUrl::from("/define/metta")]
            }
        )
    }

    #[test]
    fn create_suttaplex_search_result() {
        let key = SearchResultKey::Suttaplex {
            uid: SuttaplexUid::from("mn1"),
        };

        let search_results = SearchResults::Suttaplex {
            results: vec![SuttaplexUid::from("mn1")],
        };

        let search = CategorySearch::new(&key, &search_results);

        assert_eq!(
            search,
            CategorySearch::Suttaplex {
                search_for: SuttaplexUid::from("mn1"),
                in_results: vec![SuttaplexUid::from("mn1")]
            }
        )
    }

    #[test]
    fn text_is_found() {
        let search = CategorySearch::Text {
            search_for: TextUrl::from("/mn1/en/bodhi"),
            in_results: vec![TextUrl::from("/mn1/en/bodhi")],
        };

        assert!(search.found());
    }

    #[test]
    fn text_is_missing() {
        let search = CategorySearch::Text {
            search_for: TextUrl::from("/mn1/en/bodhi"),
            in_results: vec![],
        };

        assert!(!search.found());
    }

    #[test]
    fn dictionary_is_found() {
        let search = CategorySearch::Dictionary {
            search_for: DictionaryUrl::from("/define/metta"),
            in_results: vec![DictionaryUrl::from("/define/metta")],
        };

        assert!(search.found());
    }

    #[test]
    fn dictionary_is_missing() {
        let search = CategorySearch::Dictionary {
            search_for: DictionaryUrl::from("/define/metta"),
            in_results: vec![DictionaryUrl::from("/define/dosa")],
        };

        assert!(!search.found());
    }

    #[test]
    fn suttaplex_is_found() {
        let search = CategorySearch::Suttaplex {
            search_for: SuttaplexUid::from("mn1"),
            in_results: vec![SuttaplexUid::from("mn1"), SuttaplexUid::from("mn2")],
        };

        assert!(search.found());
    }

    #[test]
    fn suttaplex_is_not_found() {
        let search = CategorySearch::Suttaplex {
            search_for: SuttaplexUid::from("mn1"),
            in_results: vec![SuttaplexUid::from("mn2"), SuttaplexUid::from("mn3")],
        };

        assert!(!search.found());
    }

    #[test]
    fn suttaplex_has_rank() {
        let search = CategorySearch::Suttaplex {
            search_for: SuttaplexUid::from("mn3"),
            in_results: vec![SuttaplexUid::from("mn2"), SuttaplexUid::from("mn3")],
        };

        assert_eq!(search.rank(), Some(2));
    }

    #[test]
    fn missing_suttaplex_has_no_rank() {
        let search = CategorySearch::Suttaplex {
            search_for: SuttaplexUid::from("mn3"),
            in_results: Vec::new(),
        };

        assert_eq!(search.rank(), None);
    }

    #[test]
    fn text_has_rank() {
        let search = CategorySearch::Text {
            search_for: TextUrl::from("/mn1/en/bodhi"),
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
            search_for: TextUrl::from("/mn1/en/bodhi"),
            in_results: Vec::new(),
        };

        assert_eq!(search.rank(), None);
    }

    #[test]
    fn dictionary_has_rank() {
        let search = CategorySearch::Dictionary {
            search_for: DictionaryUrl::from("/define/metta"),
            in_results: vec![DictionaryUrl::from("/define/metta")],
        };
        assert_eq!(search.rank(), Some(1))
    }

    #[test]
    fn dictionary_has_no_rank() {
        let search = CategorySearch::Dictionary {
            search_for: DictionaryUrl::from("/define/metta"),
            in_results: Vec::new(),
        };
        assert_eq!(search.rank(), None)
    }
}
