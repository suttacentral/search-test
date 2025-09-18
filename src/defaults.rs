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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_all_none_when_using_default_method() {
        assert_eq!(
            Defaults {
                limit: None,
                site_language: None,
                restrict: None,
                selected_languages: None,
                match_partial: None,
            },
            Defaults::default()
        );
    }
}
