use serde::Deserialize;

#[derive(Deserialize)]
struct Hit {
    url: String,
}

#[derive(Deserialize)]
pub struct SearchResults {
    pub total: u16,
    hits: Vec<Hit>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_dictionary_hit() {
        let json = r#"
        {
            "total": 1,
            "hits" : [
                {
                    "url": "/define/metta"
                }
            ]
        }
        "#;
        let results: SearchResults = serde_json::from_str(json).unwrap();
        assert_eq!(results.hits[0].url, "/define/metta")
    }

    #[test]
    fn get_sutta_hit() {
        let json = r#"
        {
            "total": 1,
            "hits" : [
                {
                    "url": "/sa264/en/analayo"
                }
            ]
        }
        "#;
        let results: SearchResults = serde_json::from_str(json).unwrap();
        assert_eq!(results.hits[0].url, "/sa264/en/analayo")
    }
}
