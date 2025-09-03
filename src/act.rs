use serde::Deserialize;
use std::fmt;
use std::fmt::Display;

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
        url: String,
    },
    Text {
        uid: String,
        lang: String,
        author_uid: Option<String>,
        url: String,
    },
}

impl Display for Hit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Hit::Dictionary { url, .. } => {
                write!(f, "Dictionary hit: {url}")
            }
            Hit::Text { url, .. } => {
                write!(f, "Text hit {url}")
            }
        }
    }
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
    fn parse_dictionary_hit() {
        let json = r#"
        {
            "category": "dictionary",
            "highlight": {
                "detail" : {
                    "dictname": "dppn",
                    "word": "metta"
                }
            },
            "url": "/define/metta"
        }
        "#
        .to_string();

        if let Hit::Dictionary { url, .. } = serde_json::from_str(json.as_str()).unwrap() {
            assert_eq!(url, "/define/metta");
        } else {
            panic!("Wrong hit variant");
        };
    }

    #[test]
    fn parse_sutta_hit() {
        let json = r#"
        {
            "uid": "sa264",
            "lang": "en",
            "author_uid": "analayo",
            "url": "/sa264/en/analayo"
        }
        "#
        .to_string();

        if let Hit::Text { url, .. } = serde_json::from_str(json.as_str()).unwrap() {
            assert_eq!(url, "/sa264/en/analayo");
        } else {
            panic!("Wrong hit variant");
        };
    }

    #[test]
    fn parse_guide() {
        let json = r#"
        {
            "uid": "sn-guide-sujato",
            "lang": "en",
            "author_uid": null,
            "url": "/sn-guide-sujato"
        }
        "#
        .to_string();

        let guide_hit: Hit = serde_json::from_str(json.as_str()).unwrap();

        if let Hit::Text { url, .. } = serde_json::from_str(json.as_str()).unwrap() {
            assert_eq!(url, "/sn-guide-sujato");
        } else {
            panic!("Wrong hit variant");
        };
    }

    #[test]
    fn parse_licensing() {
        let json = r#"
        {
            "uid": "licensing",
            "lang": "en",
            "author_uid": null,
            "url": "/licensing"
        }
        "#
        .to_string();

        let licensing_hit: Hit = serde_json::from_str(json.as_str()).unwrap();

        if let Hit::Text { url, .. } = serde_json::from_str(json.as_str()).unwrap() {
            assert_eq!(url, "/licensing");
        } else {
            panic!("Wrong hit variant");
        };
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
