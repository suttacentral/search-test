use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Hit {
    Dictionary { url: String, category: String },
    Sutta { url: String, uid: String },
}

#[derive(Deserialize)]
pub struct SearchResults {
    pub total: u16,
    pub hits: Vec<Hit>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with_hits() -> SearchResults {
        let json = r#"
        {
            "total": 1,
            "hits" : [
                {
                    "url": "/define/metta",
                    "category": "dictionary"
                },
                {
                    "url": "/sa264/en/analayo",
                    "uid": "sa264"
                }
            ]
        }
        "#
        .to_string();
        serde_json::from_str(json.as_str()).unwrap()
    }

    #[test]
    fn get_dictionary_hit() {
        let results = with_hits();
        assert!(matches!(results.hits[0], Hit::Dictionary { .. }))
    }

    #[test]
    fn get_sutta_hit() {
        let results = with_hits();
        assert!(matches!(results.hits[1], Hit::Sutta { .. }))
    }
}
