use crate::identifiers::TextUrl;
use anyhow::Result;

use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Text {
    uid: String,
    lang: String,
    author_uid: Option<String>,
    url: TextUrl,
}

#[derive(Deserialize, Debug)]
pub struct TopLevel {
    hits: Vec<Text>,
}

fn texts(json: &str) -> Result<Vec<TextUrl>> {
    let top_level: TopLevel = serde_json::from_str(json)?;
    let urls = top_level.hits.iter().map(|hit| hit.url.clone()).collect();
    Ok(urls)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_text_result() {
        let json = r#"
        {
            "hits": [
                {
                    "uid": "mn1",
                    "lang": "en",
                    "author_uid": "sujato",
                    "url": "/mn1/en/sujato"
                }
            ]
        }
        "#;

        assert_eq!(texts(json).unwrap(), vec![TextUrl::from("/mn1/en/sujato")])
    }
}
