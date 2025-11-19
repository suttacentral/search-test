use crate::identifiers::{DictionaryUrl, SuttaplexUid, TextUrl};
use anyhow::{Error, Result};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GeneralSearchResults {
    pub text: Vec<TextUrl>,
    pub dictionary: Vec<DictionaryUrl>,
    pub fuzzy_dictionary: Vec<DictionaryUrl>,
    pub suttaplex: Vec<SuttaplexUid>,
}

impl TryFrom<&str> for GeneralSearchResults {
    type Error = Error;

    fn try_from(_value: &str) -> Result<Self> {
        Ok(GeneralSearchResults::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_text_result() {
        let json = r#"
        {
            "uid": "sa264",
            "lang": "en",
            "author_uid": "analayo",
            "url": "/sa264/en/analayo"
        }
        "#;

        assert_eq!(
            GeneralSearchResults::try_from(json).unwrap(),
            GeneralSearchResults {
                text: vec![],
                ..GeneralSearchResults::default()
            },
        )
    }
}
