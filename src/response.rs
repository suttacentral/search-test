use crate::identifiers::{DictionaryUrl, SearchResultKey, SuttaplexUid, TextUrl};
use serde::Deserialize;
use std::fmt;
use std::fmt::Display;

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Hit {
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

impl Display for Hit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Hit::Text { url, .. } => write!(f, "Text hit: {url}"),
            Hit::Dictionary { url, .. } => write!(f, "Dictionary hit: {url}"),
        }
    }
}

impl Hit {
    fn text_url(&self) -> Option<TextUrl> {
        if let Hit::Text { url, .. } = self {
            Some(url.clone())
        } else {
            None
        }
    }

    fn dictionary_url(&self) -> Option<DictionaryUrl> {
        if let Hit::Dictionary { url, .. } = self {
            Some(url.clone())
        } else {
            None
        }
    }

    #[allow(unused)]
    fn new_text(uid: &str, lang: &str, author: &str) -> Hit {
        let url = format!("/{uid}/{lang}/{author}");

        Hit::Text {
            uid: String::from(uid),
            lang: String::from(lang),
            author_uid: Some(String::from(author)),
            url: TextUrl::from(url.as_str()),
        }
    }

    #[allow(unused)]
    fn new_dictionary(word: &str) -> Hit {
        let url = format!("/define/{word}");

        Hit::Dictionary {
            category: String::from("dictionary"),
            url: DictionaryUrl::from(url.as_str()),
        }
    }
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
    total: u16,
    hits: Vec<Hit>,
    suttaplex: Vec<Suttaplex>,
    fuzzy_dictionary: Vec<FuzzyDictionary>,
}

impl SearchResponse {
    pub fn text_hits(&self) -> impl Iterator<Item = TextUrl> {
        self.hits.iter().filter_map(|h| h.text_url())
    }

    pub fn dictionary_hits(&self) -> impl Iterator<Item = DictionaryUrl> {
        self.hits.iter().filter_map(|h| h.dictionary_url())
    }

    pub fn fuzzy_dictionary_hits(&self) -> impl Iterator<Item = DictionaryUrl> {
        self.fuzzy_dictionary.iter().map(|d| d.url.clone())
    }

    pub fn suttaplex_hits(&self) -> impl Iterator<Item = SuttaplexUid> {
        self.suttaplex.iter().map(|s| s.uid.clone())
    }
}

impl Display for SearchResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} results", self.total)?;

        self.dictionary_hits()
            .try_for_each(|url| writeln!(f, "Dictionary hit: {url}"))?;

        self.fuzzy_dictionary_hits()
            .try_for_each(|url| writeln!(f, "Fuzzy dictionary hit: {url}"))?;

        self.text_hits()
            .try_for_each(|hit| writeln!(f, "Text hit: {hit}"))?;

        self.suttaplex_hits()
            .try_for_each(|uid| writeln!(f, "Suttaplex hit: {uid}"))?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct SearchResults {
    pub text: Vec<TextUrl>,
    pub dictionary: Vec<DictionaryUrl>,
    pub suttaplex: Vec<SuttaplexUid>,
}

impl SearchResults {
    pub fn new(response: SearchResponse) -> Self {
        SearchResults {
            text: response.text_hits().collect(),
            suttaplex: response.suttaplex_hits().collect(),
            dictionary: response
                .dictionary_hits()
                .chain(response.fuzzy_dictionary_hits())
                .collect(),
        }
    }

    #[allow(unused)]
    pub fn rank(&self, result: &SearchResultKey) -> Option<usize> {
        match result {
            SearchResultKey::Text { url } => self.rank_text(url),
            SearchResultKey::Dictionary { url } => self.rank_dictionary(url),
            SearchResultKey::Suttaplex { uid } => self.rank_suttaplex(uid),
        }
    }

    fn rank_text(&self, url: &TextUrl) -> Option<usize> {
        self.text
            .iter()
            .position(|h| h == url)
            .map(|position| position + 1)
    }

    fn rank_dictionary(&self, url: &DictionaryUrl) -> Option<usize> {
        self.dictionary
            .iter()
            .position(|h| h == url)
            .map(|position| position + 1)
    }

    fn rank_suttaplex(&self, uri: &SuttaplexUid) -> Option<usize> {
        self.suttaplex
            .iter()
            .position(|hit| hit == uri)
            .map(|position| position + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;

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

        let response: SearchResponse = serde_json::from_str(json)
            .context("Failed to parse JSON.")
            .unwrap();
        let suttaplex = response.suttaplex_hits().next().unwrap();
        assert_eq!(suttaplex, SuttaplexUid::from("an11.15"));
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

        let response: SearchResponse = serde_json::from_str(json)
            .context("Failed to parse JSON.")
            .unwrap();
        assert_eq!(
            response.fuzzy_dictionary_hits().next().unwrap(),
            DictionaryUrl::from("/define/anupacchinnā")
        );
    }

    #[test]
    fn rank_text_hits() {
        let response = SearchResponse {
            total: 0,
            suttaplex: Vec::new(),
            fuzzy_dictionary: Vec::new(),
            hits: vec![
                Hit::new_text("mn1", "en", "bodhi"),
                Hit::new_dictionary("metta"),
                Hit::new_text("mn2", "en", "bodhi"),
            ],
        };

        let result = SearchResults::new(response);

        let mn1 = SearchResultKey::Text {
            url: TextUrl::from("/mn1/en/bodhi"),
        };
        let mn2 = SearchResultKey::Text {
            url: TextUrl::from("/mn2/en/bodhi"),
        };
        let missing = SearchResultKey::Text {
            url: TextUrl::from("/mn1/fr/bodhi"),
        };

        assert_eq!(result.rank(&mn1), Some(1));
        assert_eq!(result.rank(&mn2), Some(2));
        assert_eq!(result.rank(&missing), None);
    }

    #[test]
    fn rank_dictionary_hits() {
        let response = SearchResponse {
            total: 0,
            suttaplex: Vec::new(),
            fuzzy_dictionary: vec![FuzzyDictionary {
                url: DictionaryUrl::from("/define/nibbana"),
            }],
            hits: vec![
                Hit::new_dictionary("metta"),
                Hit::new_text("mn1", "en", "bodhi"),
                Hit::new_dictionary("dosa"),
            ],
        };

        let result = SearchResults::new(response);

        let metta = SearchResultKey::Dictionary {
            url: DictionaryUrl::from("/define/metta"),
        };
        let dosa = SearchResultKey::Dictionary {
            url: DictionaryUrl::from("/define/dosa"),
        };
        let nibbana = SearchResultKey::Dictionary {
            url: DictionaryUrl::from("/define/nibbana"),
        };
        let brahma = SearchResultKey::Dictionary {
            url: DictionaryUrl::from("/define/brahma"),
        };

        assert_eq!(result.rank(&metta), Some(1));
        assert_eq!(result.rank(&dosa), Some(2));
        assert_eq!(result.rank(&nibbana), Some(3));
        assert_eq!(result.rank(&brahma), None);
    }

    #[test]
    fn rank_suttaplex_hits() {
        let response = SearchResponse {
            total: 0,
            hits: Vec::new(),
            fuzzy_dictionary: Vec::new(),
            suttaplex: vec![Suttaplex::from("mn1"), Suttaplex::from("mn2")],
        };

        let result = SearchResults::new(response);

        let mn1 = SearchResultKey::Suttaplex {
            uid: SuttaplexUid::from("mn1"),
        };
        let mn2 = SearchResultKey::Suttaplex {
            uid: SuttaplexUid::from("mn2"),
        };
        let mn3 = SearchResultKey::Suttaplex {
            uid: SuttaplexUid::from("mn3"),
        };

        assert_eq!(result.rank(&mn1), Some(1));
        assert_eq!(result.rank(&mn2), Some(2));
        assert_eq!(result.rank(&mn3), None);
    }
}
