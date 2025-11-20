use crate::identifiers::DictionaryUrl;
use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct FuzzyDictionaryHit {
    url: DictionaryUrl,
}

#[derive(Deserialize, Debug)]
struct FuzzyDictionaryHits {
    fuzzy_dictionary: Vec<FuzzyDictionaryHit>,
}

pub fn fuzzy_dictionary_results(json: &str) -> Result<Vec<DictionaryUrl>> {
    let hits: FuzzyDictionaryHits = serde_json::from_str(json)?;
    let urls = hits
        .fuzzy_dictionary
        .iter()
        .map(|hit| hit.url.clone())
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
            "fuzzy_dictionary": []
        }
        "#;

        assert_eq!(fuzzy_dictionary_results(json).unwrap(), Vec::new())
    }

    #[test]
    fn single_result() {
        let json = r#"
        {
            "fuzzy_dictionary": [
                {
                    "url": "/define/metta",
                    "category": "dictionary"
                }
            ]
        }
        "#;

        assert_eq!(
            fuzzy_dictionary_results(json).unwrap(),
            vec![DictionaryUrl::from("/define/metta")]
        )
    }

    #[test]
    fn two_results() {
        let json = r#"
        {
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
            fuzzy_dictionary_results(json).unwrap(),
            vec![
                DictionaryUrl::from("/define/metta"),
                DictionaryUrl::from("/define/dosa")
            ]
        )
    }
}
