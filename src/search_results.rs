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
    use crate::identifiers::{SearchResultKey, TextUrl};
    use crate::search_results::SearchResults;

    #[test]
    fn rank_text_hits() {
        let mn1 = SearchResultKey::Text {
            url: TextUrl::from("/mn1/en/bodhi"),
        };
        let mn2 = SearchResultKey::Text {
            url: TextUrl::from("/mn2/en/bodhi"),
        };
        let missing = SearchResultKey::Text {
            url: TextUrl::from("/mn1/fr/bodhi"),
        };

        let results = SearchResults {
            text: vec![
                TextUrl::from("/mn1/en/bodhi"),
                TextUrl::from("/mn2/en/bodhi"),
            ],
            dictionary: Vec::new(),
            suttaplex: Vec::new(),
        };

        assert_eq!(results.rank(&mn1), Some(1));
        assert_eq!(results.rank(&mn2), Some(2));
        assert_eq!(results.rank(&missing), None);
    }
}
