use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Hit {
    Dictionary { url: String, category: String },
    Sutta { url: String, uid: String },
}

#[derive(Deserialize, Debug)]
pub struct Suttaplex {
    pub uid: String,
}

#[derive(Deserialize, Debug)]
pub struct SearchResults {
    pub total: u16,
    pub hits: Vec<Hit>,
    pub suttaplex: Vec<Suttaplex>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_dictionary_hit() {
        let json = r#"
        {
            "total": 1,
            "suttaplex" : [],
            "hits" : [
                { "url": "/define/metta", "category": "dictionary" },
                { "url": "/sa264/en/analayo", "uid": "sa264" }
            ]
        }
        "#
        .to_string();

        let results: SearchResults = serde_json::from_str(json.as_str()).unwrap();

        assert!(matches!(results.hits[0], Hit::Dictionary { .. }))
    }

    #[test]
    fn finds_sutta_hit() {
        let json = r#"
        {
            "total": 1,
            "suttaplex" : [],
            "hits" : [
                { "url": "/define/metta", "category": "dictionary" },
                { "url": "/sa264/en/analayo", "uid": "sa264" }
            ]
        }
        "#
        .to_string();

        let results: SearchResults = serde_json::from_str(json.as_str()).unwrap();

        assert!(matches!(results.hits[1], Hit::Sutta { .. }))
    }

    #[test]
    fn finds_two_sutta_hits() {
        let json = r#"
        {
            "total": 1,
            "suttaplex" : [],
            "hits" : [
                { "url": "/sa264/en/analayo", "uid": "sa264" },
                { "url": "/snp1.3/en/mills", "uid": "snp1.3" }
            ]
        }
        "#
        .to_string();

        let results: SearchResults = serde_json::from_str(json.as_str()).unwrap();

        assert_eq!(results.hits.len(), 2);
    }

    #[test]
    fn finds_two_sutta_and_one_dictionary_hit() {
        let json = r#"
        {
            "total": 1,
            "suttaplex" : [],
            "hits" : [
                { "url": "/define/metta", "category": "dictionary" },
                { "url": "/sa264/en/analayo", "uid": "sa264" },
                { "url": "/snp1.3/en/mills", "uid": "snp1.3" }
            ]
        }
        "#
        .to_string();

        let results: SearchResults = serde_json::from_str(json.as_str()).unwrap();

        assert_eq!(results.hits.len(), 3);
    }

    #[test]
    fn finds_a_suttaplex() {
        let json = r#"
        {
            "total": 1,
            "hits" : [],
            "suttaplex" : [
                { "uid": "an11.15" }
            ]
        }
        "#
        .to_string();

        let results: SearchResults = serde_json::from_str(json.as_str()).unwrap();
        assert_eq!(results.suttaplex[0].uid, "an11.15");
    }
}
