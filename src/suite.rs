use anyhow::Context;
use saphyr::LoadableYamlNode;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Settings {
    endpoint: String,
    limit: usize,
    site_language: String,
    restrict: String,
    selected_languages: Vec<String>,
    match_partial: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct TestCase {
    description: Option<String>,
    query: String,
    selected_languages: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct TestSuite {
    settings: Settings,
    test_case: Vec<TestCase>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suite_with_one_test_case() {
        let suite: TestSuite = toml::from_str(
            r#"
            [settings]
            endpoint = "http://localhost/api/search/instant"
            limit = 50
            site-language = "en"
            restrict = "all"
            selected-languages = ["en", "pli"]
            match-partial = false

            [[test-case]]
            description = "Search for the metta sutta in English and Pali"
            query = "metta"
            selected-languages = ["pli", "en"]
        "#,
        )
        .unwrap();

        let expected = TestSuite {
            settings: Settings {
                endpoint: "http://localhost/api/search/instant".to_string(),
                limit: 50,
                site_language: "en".to_string(),
                restrict: "all".to_string(),
                selected_languages: vec!["en".to_string(), "pli".to_string()],
                match_partial: false,
            },
            test_case: vec![TestCase {
                description: Some("Search for the metta sutta in English and Pali".to_string()),
                query: "metta".to_string(),
                selected_languages: Some(vec!["pli".to_string(), "en".to_string()]),
            }],
        };

        assert_eq!(suite, expected);
    }
}
