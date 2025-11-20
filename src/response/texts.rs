use crate::identifiers::{DictionaryUrl, TextUrl};
use crate::response::mixed_hits::Hit;
use anyhow::Result;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct TopLevel {
    hits: Vec<Hit>,
}

pub fn texts(json: &str) -> Result<Vec<TextUrl>> {
    let top_level: TopLevel = serde_json::from_str(json)?;
    let urls = top_level
        .hits
        .iter()
        .filter_map(|hit| hit.text_url())
        .collect();
    Ok(urls)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_results() {
        let json = r#"
        {
            "hits": []
        }
        "#;

        assert_eq!(texts(json).unwrap(), Vec::new())
    }

    #[test]
    fn single_result() {
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

        assert_eq!(texts(json).unwrap(), vec![TextUrl::from("/mn1/en/sujato")])
    }

    #[test]
    fn two_results() {
        let json = r#"
        {
            "hits": [
                {
                    "uid": "mn1",
                    "lang": "en",
                    "author_uid": "sujato",
                    "url": "/mn1/en/sujato"
                },
                {
                    "uid": "mn2",
                    "lang": "en",
                    "author_uid": "sujato",
                    "url": "/mn2/en/sujato"
                }
            ]
        }
        "#;

        assert_eq!(
            texts(json).unwrap(),
            vec![
                TextUrl::from("/mn1/en/sujato"),
                TextUrl::from("/mn2/en/sujato")
            ]
        )
    }

    #[test]
    fn ignores_dictionary_result() {
        let json = r#"
        {
            "hits": [
                {
                    "uid": "mn1",
                    "lang": "en",
                    "author_uid": "sujato",
                    "url": "/mn1/en/sujato"
                },
                {
                    "category": "dictionary",
                    "url": "/define/metta"
                }
            ]
        }
        "#;

        assert_eq!(texts(json).unwrap(), vec![TextUrl::from("/mn1/en/sujato")])
    }

    #[test]
    fn ignores_other_top_level_attributes() {
        let json = r#"
        {
            "total": 80,
            "hits": [
                {
                    "uid": "mn1",
                    "lang": "en",
                    "author_uid": "sujato",
                    "url": "/mn1/en/sujato"
                }
            ],
            "suttaplex": [],
            "fuzzy_dictionary": []
        }
        "#;

        assert_eq!(texts(json).unwrap(), vec![TextUrl::from("/mn1/en/sujato")])
    }
}
