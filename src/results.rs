use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Detail {
    dictname: String,
    word: String,
}

#[derive(Deserialize, Debug)]
pub struct Highlight {
    detail: Detail,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Hit {
    Dictionary {
        highlight: Highlight,
        category: String,
    },
    Sutta {
        uid: String,
        author_uid: String,
    },
}

#[derive(Deserialize, Debug)]
pub struct Suttaplex {
    pub uid: String,
}

#[derive(Deserialize, Debug)]
pub struct FuzzyDictionary {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct SearchResults {
    pub total: u16,
    pub hits: Vec<Hit>,
    pub suttaplex: Vec<Suttaplex>,
    pub fuzzy_dictionary: Vec<FuzzyDictionary>,
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
            "fuzzy_dictionary": [],
            "hits" : [
                {
                    "url": "/define/metta",
                    "category": "dictionary",
                    "highlight": {
                        "detail" : {
                            "dictname": "dppn",
                            "word": "metta"
                        }
                    }
                }
            ]
        }
        "#
        .to_string();

        let results: SearchResults = serde_json::from_str(json.as_str()).unwrap();

        assert!(matches!(results.hits[0], Hit::Dictionary { .. }));
    }

    #[test]
    fn finds_sutta_hit() {
        let json = r#"
        {
            "total": 1,
            "suttaplex" : [],
            "fuzzy_dictionary": [],
            "hits" : [
                {
                    "uid": "sa264",
                    "author_uid": "analayo"
                }
            ]
        }
        "#
        .to_string();

        let results: SearchResults = serde_json::from_str(json.as_str()).unwrap();

        assert!(matches!(results.hits[0], Hit::Sutta { .. }));
    }

    #[test]
    fn finds_two_sutta_hits() {
        let json = r#"
        {
            "total": 1,
            "suttaplex" : [],
            "fuzzy_dictionary": [],
            "hits" : [
                { "uid": "sa264", "author_uid": "analayo" },
                { "uid": "snp1.3", "author_uid": "mills" }
            ]
        }
        "#
        .to_string();

        let results: SearchResults = serde_json::from_str(json.as_str()).unwrap();

        assert_eq!(results.hits.len(), 2);
    }

    #[test]
    fn finds_sutta_and_dictionary_hit() {
        let json = r#"
        {
            "total": 1,
            "suttaplex" : [],
            "fuzzy_dictionary": [],
            "hits" : [
                { "uid": "snp1.3", "author_uid": "mills" },
                {
                    "url": "/define/metta",
                    "category": "dictionary",
                    "highlight": {
                        "detail" : {
                            "dictname": "dppn",
                            "word": "metta"
                        }
                    }
                }
            ]
        }
        "#
        .to_string();

        let results: SearchResults = serde_json::from_str(json.as_str()).unwrap();

        assert_eq!(results.hits.len(), 2);
    }

    #[test]
    fn finds_a_suttaplex() {
        let json = r#"
        {
            "total": 1,
            "hits" : [],
            "fuzzy_dictionary": [],
            "suttaplex" : [
                { "uid": "an11.15" }
            ]
        }
        "#
        .to_string();

        let results: SearchResults = serde_json::from_str(json.as_str()).unwrap();
        assert_eq!(results.suttaplex[0].uid, "an11.15");
    }

    #[test]
    fn finds_a_fuzzy_dictionary_result() {
        let json = r#"
        {
            "total": 1,
            "hits" : [],
            "suttaplex" : [],
            "fuzzy_dictionary": [
                { "url": "/define/anupacchinnā" }
            ]
        }
        "#
        .to_string();

        let results: SearchResults = serde_json::from_str(json.as_str()).unwrap();
        assert_eq!(results.fuzzy_dictionary[0].url, "/define/anupacchinnā");
    }
}
