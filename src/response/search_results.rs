use crate::identifiers::{DictionaryUrl, SearchResultKey, SuttaplexUid, TextUrl, VolpageReference};
use crate::response::texts::text_results;
use anyhow::Result;

#[derive(Clone, Debug, PartialEq)]
pub enum SearchResults {
    Text { results: Vec<TextUrl> },
    Dictionary { results: Vec<DictionaryUrl> },
    FuzzyDictionary { results: Vec<DictionaryUrl> },
    Suttaplex { results: Vec<SuttaplexUid> },
    Volpage { results: Vec<VolpageReference> },
}

impl SearchResults {
    pub fn new(key: SearchResultKey, json: &str) -> Result<SearchResults> {
        match key {
            SearchResultKey::Text { .. } => {
                let results = text_results(json)?;
                Ok(SearchResults::Text { results })
            }
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::identifiers::{SearchResultKey, TextUrl};
    use crate::response::search_results::SearchResults;

    #[test]
    fn constructs_text_results() {
        let key = SearchResultKey::Text {
            url: TextUrl::from("/mn1/en/sujato"),
        };

        let json = r#"
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

        assert_eq!(
            SearchResults::new(key, json).unwrap(),
            SearchResults::Text {
                results: vec![TextUrl::from("/mn1/en/sujato")]
            }
        );
    }
}
