use crate::identifiers::{DictionaryUrl, SearchResultKey, SuttaplexUid, TextUrl};
use crate::response::SearchResults;

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
        match key {
            SearchResultKey::Suttaplex { uid } => Self::Suttaplex {
                search_for: uid.clone(),
                in_results: results.suttaplex.to_vec(),
            },
            SearchResultKey::Dictionary { url } => Self::Dictionary {
                search_for: url.clone(),
                in_results: results.dictionary.to_vec(),
            },
            SearchResultKey::Text { url } => Self::Text {
                search_for: url.clone(),
                in_results: results.text.to_vec(),
            },
            SearchResultKey::Volpage { reference: _ } => todo!(),
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
                in_results: vec![TextUrl::from("/mn1/en/bodhi")]
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
                in_results: vec![DictionaryUrl::from("/define/metta")]
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
