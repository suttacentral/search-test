use crate::arrange;
use reqwest::blocking::{Client, RequestBuilder};
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

impl Hit {
    pub fn url_path(&self) -> String {
        match self {
            Hit::Text { url, .. } => url.clone(),
            Hit::Dictionary { url, .. } => url.clone(),
        }
    }
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
pub struct SearchResponse {
    pub total: u16,
    pub hits: Vec<Hit>,
    pub suttaplex: Vec<Suttaplex>,
    pub fuzzy_dictionary: Vec<FuzzyDictionary>,
}

impl SearchResponse {
    pub fn dictionary_hits(&self) -> Vec<String> {
        let mut dict_hits: Vec<String> = Vec::new();
        for hit in &self.hits {
            match hit {
                Hit::Dictionary { .. } => dict_hits.push(hit.url_path()),
                _ => (),
            }
        }
        dict_hits
    }
}

impl Display for SearchResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} results", self.total)?;
        writeln!(f, "{} hits", self.hits.len())?;
        for hit in &self.hits {
            writeln!(f, "{hit}")?;
        }
        for suttaplex in &self.suttaplex {
            writeln!(f, "Suttaplex result: {}", suttaplex.uid)?;
        }
        for fuzzy in &self.fuzzy_dictionary {
            writeln!(f, "Fuzzy dictionary result: {}", fuzzy.url)?;
        }
        Ok(())
    }
}

pub fn build_request(endpoint: String, test_case: arrange::TestCase) -> RequestBuilder {
    let params = vec![
        ("limit", test_case.limit.to_string()),
        ("query", test_case.query),
        ("language", test_case.site_language),
        ("restrict", test_case.restrict),
        ("matchpartial", test_case.match_partial.to_string()),
    ];

    Client::new()
        .post(endpoint.as_str())
        .query(&params)
        .json(&test_case.selected_languages)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arrange::TestSuite;

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
    fn parse_text_hit() {
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

        let results: SearchResponse = serde_json::from_str(json.as_str()).unwrap();
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

        let results: SearchResponse = serde_json::from_str(json.as_str()).unwrap();
        assert_eq!(results.fuzzy_dictionary[0].url, "/define/anupacchinnā");
    }

    fn test_suite() -> anyhow::Result<TestSuite> {
        TestSuite::load_from_string(
            r#"
        [settings]
        endpoint = "http://localhost/api/search/instant"

        [defaults]
        limit = 1
        site-language = "en"
        restrict = "all"
        match-partial=false
        selected-languages = ["en", "pli"]

        [[test-case]]
        description = "The Simile of the Adze"
        query = "adze"
        "#,
        )
    }

    #[test]
    fn builds_correct_url() {
        let suite = test_suite().unwrap();
        let test_case = suite.test_cases().unwrap().iter().next().unwrap().clone();
        let request = build_request(suite.endpoint(), test_case).build().unwrap();
        let expected = "http://localhost/api/search/instant?limit=1&query=adze&language=en&restrict=all&matchpartial=false";
        let actual = request.url().to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn has_correct_body() {
        let suite = test_suite().unwrap();
        let test_case = suite.test_cases().unwrap().iter().next().unwrap().clone();
        let request = build_request(suite.endpoint(), test_case).build().unwrap();
        let body = request.body().unwrap().as_bytes().unwrap();
        let body_contents = str::from_utf8(body).unwrap().to_string();
        assert_eq!(body_contents, "[\"en\",\"pli\"]");
    }

    #[test]
    fn get_text_hit_path() {
        let hit = Hit::Text {
            uid: String::from("sa264"),
            lang: String::from("en"),
            author_uid: Some(String::from("analayo")),
            url: String::from("/sa264/en/analayo"),
        };

        assert_eq!(hit.url_path(), "/sa264/en/analayo");
    }

    fn dictionary_hit(word: &str, url: &str) -> Hit {
        Hit::Dictionary {
            category: String::from("dictionary"),
            highlight: Highlight {
                detail: Detail {
                    dictname: String::from("dppn"),
                    word: String::from(word),
                },
            },
            url: String::from(url),
        }
    }

    #[test]
    fn get_dictionary_hit_path() {
        let hit = dictionary_hit("metta", "/define/metta");
        assert_eq!(hit.url_path(), "/define/metta");
    }

    #[test]
    fn get_all_paths_for_dictionary_hits_from_search_response() {
        let response = SearchResponse {
            total: 0,
            suttaplex: Vec::new(),
            fuzzy_dictionary: Vec::new(),
            hits: vec![
                dictionary_hit("metta", "/define/metta"),
                dictionary_hit("dosa", "/define/dosa"),
                dictionary_hit("brahma", "/define/brahma"),
            ],
        };
        let expected = vec![
            String::from("/define/metta"),
            String::from("/define/dosa"),
            String::from("/define/brahma"),
        ];
        assert_eq!(expected, response.dictionary_hits());
    }
}
