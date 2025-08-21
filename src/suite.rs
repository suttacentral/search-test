use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Settings {
    pub endpoint: String,
    pub delay: usize,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Defaults {
    pub limit: Option<usize>,
    pub site_language: Option<String>,
    pub restrict: Option<String>,
    pub selected_languages: Option<Vec<String>>,
    pub match_partial: Option<bool>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct TestCase {
    pub query: String,
    pub description: Option<String>,
    pub limit: Option<usize>,
    pub site_language: Option<String>,
    pub restrict: Option<String>,
    pub selected_languages: Option<Vec<String>>,
    pub match_partial: Option<bool>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct TestSuite {
    pub settings: Settings,
    pub defaults: Defaults,
    #[serde[rename = "test-case"]]
    pub test_cases: Vec<TestCase>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_specification() {
        let suite: TestSuite = toml::from_str(
            r#"
            [settings]
            endpoint = "http://localhost/api/search/instant"
            delay = 3000

            [defaults]
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
                delay: 3000,
            },
            defaults: Defaults {
                limit: Some(50),
                site_language: Some("en".to_string()),
                restrict: Some("all".to_string()),
                selected_languages: Some(vec!["en".to_string(), "pli".to_string()]),
                match_partial: Some(false),
            },
            test_cases: vec![TestCase {
                description: Some("Search for the metta sutta in English and Pali".to_string()),
                query: "metta".to_string(),
                selected_languages: Some(vec!["pli".to_string(), "en".to_string()]),
                limit: None,
                site_language: None,
                restrict: None,
                match_partial: None,
            }],
        };

        assert_eq!(suite, expected);
    }
}
