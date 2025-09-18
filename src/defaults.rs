use serde::Deserialize;

#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Defaults {
    pub limit: Option<usize>,
    pub site_language: Option<String>,
    pub restrict: Option<String>,
    pub selected_languages: Option<Vec<String>>,
    pub match_partial: Option<bool>,
}
