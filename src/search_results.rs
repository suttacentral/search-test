use crate::identifiers::{DictionaryUrl, SearchResultKey, SuttaplexUid, TextUrl};
use crate::response::SearchResponse;

pub struct SearchResults {
    pub text: Vec<TextUrl>,
    pub dictionary: Vec<DictionaryUrl>,
    pub suttaplex: Vec<SuttaplexUid>,
}

impl From<SearchResponse> for SearchResults {
    fn from(response: SearchResponse) -> Self {
        SearchResults {
            text: response.text_hits().collect(),
            suttaplex: response.suttaplex_hits().collect(),
            dictionary: response
                .dictionary_hits()
                .chain(response.fuzzy_dictionary_hits())
                .collect(),
        }
    }
}

impl SearchResults {
    #[allow(unused)]
    pub fn rank(&self, result: &SearchResultKey) -> Option<usize> {
        match result {
            SearchResultKey::Text { url } => self.rank_text(&url),
            SearchResultKey::Dictionary { url } => todo!(),
            SearchResultKey::Suttaplex { uid } => todo!(),
        }
    }

    fn rank_text(&self, url: &TextUrl) -> Option<usize> {
        self.text
            .iter()
            .position(|h| h == url)
            .map(|position| position + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identifiers::{DictionaryUrl, SearchResultKey, TextUrl};
    use crate::response::{FuzzyDictionary, Hit, SearchResponse};
    use crate::search_results::SearchResults;

    #[test]
    fn rank_text_hits() {
        let response = SearchResponse {
            total: 0,
            suttaplex: Vec::new(),
            fuzzy_dictionary: Vec::new(),
            hits: vec![
                Hit::new_text("mn1", "en", "bodhi"),
                Hit::new_dictionary("metta"),
                Hit::new_text("mn2", "en", "bodhi"),
            ],
        };
        let result = SearchResults::from(response);

        let mn1 = SearchResultKey::Text {
            url: TextUrl::from("/mn1/en/bodhi"),
        };
        let mn2 = SearchResultKey::Text {
            url: TextUrl::from("/mn2/en/bodhi"),
        };
        let missing = SearchResultKey::Text {
            url: TextUrl::from("/mn1/fr/bodhi"),
        };

        assert_eq!(result.rank(&mn1), Some(1));
        assert_eq!(result.rank(&mn2), Some(2));
        assert_eq!(result.rank(&missing), None);
    }

    #[test]
    fn rank_dictionary_hits() {
        let response = SearchResponse {
            total: 0,
            suttaplex: Vec::new(),
            fuzzy_dictionary: vec![FuzzyDictionary {
                url: DictionaryUrl::from("/define/nibbana"),
            }],
            hits: vec![
                Hit::new_dictionary("metta"),
                Hit::new_text("mn1", "en", "bodhi"),
                Hit::new_dictionary("dosa"),
            ],
        };

        let metta = SearchResultKey::Dictionary {
            url: DictionaryUrl::from("/define/metta"),
        };
        let dosa = SearchResultKey::Dictionary {
            url: DictionaryUrl::from("/define/dosa"),
        };
        let nibbana = SearchResultKey::Dictionary {
            url: DictionaryUrl::from("/define/nibbana"),
        };
        let brahma = SearchResultKey::Dictionary {
            url: DictionaryUrl::from("/define/brahma"),
        };

        assert_eq!(response.rank(metta), Some(1));
        assert_eq!(response.rank(dosa), Some(2));
        assert_eq!(response.rank(nibbana), Some(3));
        assert_eq!(response.rank(brahma), None);
    }
}
