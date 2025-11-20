use crate::identifiers::DictionaryUrl;
use crate::response::mixed_hits::TopLevel;
use anyhow::Result;

pub fn dictionary_results(json: &str) -> Result<Vec<DictionaryUrl>> {
    let top_level: TopLevel = serde_json::from_str(json)?;
    let urls = top_level
        .hits
        .iter()
        .filter_map(|hit| hit.dictionary_url())
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

        assert_eq!(dictionary_results(json).unwrap(), Vec::new())
    }

    #[test]
    fn single_result() {
        let json = r#"
        {
            "hits": [
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
}
