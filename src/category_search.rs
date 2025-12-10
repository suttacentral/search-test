use crate::identifiers::{DictionaryUrl, SuttaplexUid, TextUrl};
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
    pub fn new(results: &SearchResults) -> Self {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_text() {
        let search_results = SearchResults::Text {
            expected: TextUrl::from("/mn1/en/bodhi"),
            results: vec![TextUrl::from("/mn1/en/bodhi")],
        };

        assert_eq!(
            CategorySearch::new(&search_results),
            CategorySearch::Text {
                expected: TextUrl::from("/mn1/en/bodhi"),
                in_results: vec![TextUrl::from("/mn1/en/bodhi")]
            }
        );
    }

    #[test]
    fn new_dictionary() {
        let search_results = SearchResults::Dictionary {
            expected: DictionaryUrl::from("/define/metta"),
            results: vec![DictionaryUrl::from("/define/metta")],
        };

        assert_eq!(
            CategorySearch::new(&search_results),
            CategorySearch::Dictionary {
                expected: DictionaryUrl::from("/define/metta"),
                in_results: vec![DictionaryUrl::from("/define/metta")]
            }
        );
    }

    #[test]
    fn new_suttaplex() {
        let search_results = SearchResults::Suttaplex {
            expected: SuttaplexUid::from("mn1"),
            results: vec![SuttaplexUid::from("mn1")],
        };

        assert_eq!(
            CategorySearch::new(&search_results),
            CategorySearch::Suttaplex {
                expected: SuttaplexUid::from("mn1"),
                in_results: vec![SuttaplexUid::from("mn1")]
            }
        );
    }
}
