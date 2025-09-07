use crate::arrange;
use crate::identifiers::{DictionaryUrl, SearchResultIdentifier, SuttaplexUid, TextUrl};
use anyhow::{Context, Result};
use reqwest::blocking::{Client, RequestBuilder};
use serde::Deserialize;
use std::fmt;
use std::fmt::Display;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Hit {
    Dictionary {
        category: String,
        url: DictionaryUrl,
    },
    Text {
        uid: String,
        lang: String,
        author_uid: Option<String>,
        url: TextUrl,
    },
}

#[derive(Deserialize, Debug)]
struct Suttaplex {
    uid: SuttaplexUid,
}

#[derive(Deserialize, Debug)]
struct FuzzyDictionary {
    url: DictionaryUrl,
}

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    pub total: u16,
    hits: Vec<Hit>,
    suttaplex: Vec<Suttaplex>,
    fuzzy_dictionary: Vec<FuzzyDictionary>,
}

impl SearchResponse {
    pub fn rank(&self, id: SearchResultIdentifier) -> Option<u8> {
        match id {
            SearchResultIdentifier::Text { url } => Some(1),
            _ => None,
        }
    }

    pub fn dictionary_hits(&self) -> Vec<DictionaryUrl> {
        let mut dict_hits: Vec<DictionaryUrl> = Vec::new();
        for hit in &self.hits {
            if let Hit::Dictionary { url, .. } = hit {
                dict_hits.push(url.clone());
            }
        }
        dict_hits
    }

    pub fn text_hits(&self) -> Vec<TextUrl> {
        let mut dict_hits: Vec<TextUrl> = Vec::new();
        for hit in &self.hits {
            if let Hit::Text { url, .. } = hit {
                dict_hits.push(url.clone());
            }
        }
        dict_hits
    }

    pub fn suttaplexes(&self) -> Vec<SuttaplexUid> {
        self.suttaplex.iter().map(|s| s.uid.clone()).collect()
    }

    pub fn fuzzy_dictionary_hits(&self) -> Vec<DictionaryUrl> {
        self.fuzzy_dictionary
            .iter()
            .map(|d| d.url.clone())
            .collect()
    }

    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).context("Failed to parse JSON.")
    }
}

impl Display for SearchResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} results", self.total)?;

        self.dictionary_hits()
            .iter()
            .try_for_each(|url| writeln!(f, "Dictionary hit: {url}"))?;

        self.fuzzy_dictionary_hits()
            .iter()
            .try_for_each(|url| writeln!(f, "Fuzzy dictionary hit: {url}"))?;

        self.text_hits()
            .iter()
            .try_for_each(|url| writeln!(f, "Text hit: {url}"))?;

        self.suttaplexes()
            .iter()
            .try_for_each(|uid| writeln!(f, "Suttaplex hit: {uid}"))?;

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
    use crate::identifiers::SearchResultIdentifier;

    impl From<&str> for Suttaplex {
        fn from(value: &str) -> Self {
            Self {
                uid: SuttaplexUid::from(value),
            }
        }
    }

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
            assert_eq!(url, DictionaryUrl::from("/define/metta"));
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
            assert_eq!(url, TextUrl::from("/sa264/en/analayo"));
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
            assert_eq!(url, TextUrl::from("/sn-guide-sujato"));
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
            assert_eq!(url, TextUrl::from("/licensing"));
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
        "#;

        let response = SearchResponse::from_json(json).unwrap();
        assert_eq!(response.suttaplexes()[0], SuttaplexUid::from("an11.15"));
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
        "#;

        let response = SearchResponse::from_json(json).unwrap();
        assert_eq!(
            response.fuzzy_dictionary_hits()[0],
            DictionaryUrl::from("/define/anupacchinnā")
        );
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

    fn dictionary_hit(word: &str, url: &str) -> Hit {
        Hit::Dictionary {
            category: String::from("dictionary"),
            url: DictionaryUrl::from(url),
        }
    }

    fn text_hit(uid: &str, lang: &str, author: &str) -> Hit {
        let url = format!("/{uid}/{lang}/{author}");
        Hit::Text {
            uid: String::from(uid),
            lang: String::from(lang),
            author_uid: Some(String::from(author)),
            url: TextUrl::from(url.as_str()),
        }
    }

    fn search_response_with_mixed_hits() -> SearchResponse {
        SearchResponse {
            total: 0,
            suttaplex: Vec::new(),
            fuzzy_dictionary: Vec::new(),
            hits: vec![
                dictionary_hit("metta", "/define/metta"),
                dictionary_hit("dosa", "/define/dosa"),
                text_hit("sa264", "en", "analayo"),
                dictionary_hit("brahma", "/define/brahma"),
                text_hit("mn1", "en", "bodhi"),
            ],
        }
    }

    #[test]
    fn list_dictionary_hits() {
        let response = search_response_with_mixed_hits();

        let expected = vec![
            DictionaryUrl::from("/define/metta"),
            DictionaryUrl::from("/define/dosa"),
            DictionaryUrl::from("/define/brahma"),
        ];

        assert_eq!(expected, response.dictionary_hits());
    }

    #[test]
    fn list_text_hits() {
        let response = search_response_with_mixed_hits();
        let expected = vec![
            TextUrl::from("/sa264/en/analayo"),
            TextUrl::from("/mn1/en/bodhi"),
        ];
        assert_eq!(expected, response.text_hits());
    }

    #[test]
    fn list_suttaplexes() {
        let response = SearchResponse {
            total: 0,
            hits: Vec::new(),
            fuzzy_dictionary: Vec::new(),
            suttaplex: vec![
                Suttaplex::from("mn1"),
                Suttaplex::from("mn2"),
                Suttaplex::from("mn3"),
            ],
        };

        let expected = vec![
            SuttaplexUid::from("mn1"),
            SuttaplexUid::from("mn2"),
            SuttaplexUid::from("mn3"),
        ];
        assert_eq!(expected, response.suttaplexes());
    }

    #[test]
    fn list_fuzzy_dictionary_hits() {
        let response = SearchResponse {
            total: 0,
            hits: Vec::new(),
            suttaplex: Vec::new(),
            fuzzy_dictionary: vec![
                FuzzyDictionary {
                    url: DictionaryUrl::from("/define/metta"),
                },
                FuzzyDictionary {
                    url: DictionaryUrl::from("/define/dosa"),
                },
                FuzzyDictionary {
                    url: DictionaryUrl::from("/define/brahma"),
                },
            ],
        };

        let expected = vec![
            DictionaryUrl::from("/define/metta"),
            DictionaryUrl::from("/define/dosa"),
            DictionaryUrl::from("/define/brahma"),
        ];

        assert_eq!(expected, response.fuzzy_dictionary_hits());
    }

    #[test]
    fn get_rank_of_text_hit() {
        let response = SearchResponse {
            total: 0,
            suttaplex: Vec::new(),
            fuzzy_dictionary: Vec::new(),
            hits: vec![
                text_hit("sa264", "en", "analayo"),
                text_hit("mn1", "en", "bodhi"),
            ],
        };

        let top_ranked = SearchResultIdentifier::Text {
            url: TextUrl::from("/sa264/en/analayo"),
        };
        assert_eq!(response.rank(top_ranked), Some(1))
    }
}
