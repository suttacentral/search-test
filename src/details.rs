use crate::identifiers::{DictionaryUrl, SuttaplexUid, TextUrl};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct DetailsProvided {
    pub query: String,
    pub description: String,
    pub limit: Option<usize>,
    pub site_language: Option<String>,
    pub restrict: Option<String>,
    pub selected_languages: Option<Vec<String>>,
    pub match_partial: Option<bool>,
    pub expected_suttaplex: Option<SuttaplexUid>,
    pub expected_sutta: Option<TextUrl>,
    pub expected_dictionary: Option<DictionaryUrl>,
}
