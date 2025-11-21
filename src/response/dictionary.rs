use crate::identifiers::DictionaryUrl;
use crate::response::mixed_hits::{Hit, MixedHits};
use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct FuzzyDictionaryHit {
    url: DictionaryUrl,
}

#[derive(Deserialize, Debug)]
struct DictionaryHits {
    hits: Vec<Hit>,
    fuzzy_dictionary: Vec<FuzzyDictionaryHit>,
}

pub fn dictionary_results(json: &str) -> Result<Vec<DictionaryUrl>> {
    let hits: DictionaryHits = serde_json::from_str(json)?;
    let mut urls: Vec<DictionaryUrl> = hits
        .hits
        .iter()
        .filter_map(|hit| hit.dictionary_url())
        .collect();

    let fuzzy_urls: Vec<DictionaryUrl> = hits
        .fuzzy_dictionary
        .iter()
        .map(|hit| hit.url.clone())
        .collect();

    urls.extend(fuzzy_urls);

    Ok(urls)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_results() {
        let json = r#"
        {
            "hits": [],
            "fuzzy_dictionary": []
        }
        "#;

        assert_eq!(dictionary_results(json).unwrap(), Vec::new())
    }

    #[test]
    fn normal_result_only() {
        let json = r#"
        {
            "hits": [
                {
                    "url": "/define/metta",
                    "category": "dictionary"
                }
            ],
            "fuzzy_dictionary": []
        }
        "#;

        assert_eq!(
            dictionary_results(json).unwrap(),
            vec![DictionaryUrl::from("/define/metta")]
        )
    }

    #[test]
    fn two_normal_results() {
        let json = r#"
        {
            "hits": [
                {
                    "url": "/define/metta",
                    "category": "dictionary"
                },
                {
                    "url": "/define/dosa",
                    "category": "dictionary"
                }
            ],
            "fuzzy_dictionary": []
        }
        "#;

        assert_eq!(
            dictionary_results(json).unwrap(),
            vec![
                DictionaryUrl::from("/define/metta"),
                DictionaryUrl::from("/define/dosa")
            ]
        )
    }

    #[test]
    fn one_fuzzy_result() {
        let json = r#"
        {
            "hits" : [],
            "fuzzy_dictionary": [
                {
                    "url": "/define/metta",
                    "category": "dictionary"
                }
            ]
        }
        "#;

        assert_eq!(
            dictionary_results(json).unwrap(),
            vec![DictionaryUrl::from("/define/metta")]
        )
    }

    #[test]
    fn two_fuzzy_results() {
        let json = r#"
        {
            "hits" : [],
            "fuzzy_dictionary": [
                {
                    "url": "/define/metta",
                    "category": "dictionary"
                },
                {
                    "url": "/define/dosa",
                    "category": "dictionary"
                }
            ]
        }
        "#;

        assert_eq!(
            dictionary_results(json).unwrap(),
            vec![
                DictionaryUrl::from("/define/metta"),
                DictionaryUrl::from("/define/dosa")
            ]
        )
    }

    #[test]
    fn one_of_each_result() {
        let json = r#"
        {
            "hits" : [
            {
                    "url": "/define/metta",
                    "category": "dictionary"
                }
            ],
            "fuzzy_dictionary": [
                {
                    "url": "/define/dosa",
                    "category": "dictionary"
                }
            ]
        }
        "#;

        assert_eq!(
            dictionary_results(json).unwrap(),
            vec![
                DictionaryUrl::from("/define/metta"),
                DictionaryUrl::from("/define/dosa")
            ]
        )
    }
}
