use anyhow::Context;
use saphyr::LoadableYamlNode;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Settings {
    endpoint: String,
    limit: usize,
    site_language: String,
    restrict: String,
    selected_languages: Vec<String>,
    match_partial: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TestCase {
    description: Option<String>,
    query: String,
    selected_languages: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TestSuite {
    settings: Settings,
    // test_cases: Vec<TestCase>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_all_settings() {
        let suite: TestSuite = toml::from_str(
            r#"
            [settings]
            endpoint = "http://localhost/api/search/instant"
            limit = 50
            site-language = "en"
            restrict = "all"
            selected-languages = ["en", "pli"]
            match-partial = false
        "#,
        )
        .unwrap();

        assert_eq!(
            suite.settings.endpoint,
            "http://localhost/api/search/instant"
        );
    }
}
