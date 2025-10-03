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
                in_sequence: results.suttaplex.iter().cloned().collect(),
            },
            SearchResultKey::Dictionary { url } => todo!(),
            SearchResultKey::Text { url } => todo!(),
        }
    }

    pub fn found(&self) -> bool {
        match self {
            Self::Suttaplex {
                search_for,
                in_sequence,
            } => in_sequence.contains(&search_for),
            _ => todo!(),
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
}
