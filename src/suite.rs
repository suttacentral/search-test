use anyhow::{Context, Error};
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
    #[serde[rename = "test-case"]]
    test_cases: Vec<TestCase>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_settings_one_test_case() {
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
            test_cases: vec![TestCase {
                description: Some("Search for the metta sutta in English and Pali".to_string()),
                query: "metta".to_string(),
                selected_languages: Some(vec!["pli".to_string(), "en".to_string()]),
            }],
        };

        assert_eq!(suite, expected);
    }

    #[test]
    fn missing_test_cases_gives_an_error() {
        let suite = toml::from_str::<TestSuite>(
            r#"
            [settings]
            endpoint = "http://localhost/api/search/instant"
            limit = 50
            site-language = "en"
            restrict = "all"
            selected-languages = ["en", "pli"]
            match-partial = false
        "#,
        );

        match suite {
            Err(error) => assert_eq!(error.message(), "missing field `test-case`"),
            Ok(_) => panic!("Did not get expected error."),
        }
    }

    #[test]
    fn missing_setting_gives_an_error() {
        let suite = toml::from_str::<TestSuite>(
            r#"
            [settings]
            # endpoint = "http://localhost/api/search/instant"
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
        );

        match suite {
            Err(error) => assert_eq!(error.message(), "missing field `endpoint`"),
            Ok(_) => panic!("Did not get expected error."),
        }
    }

    #[test]
    fn missing_query_gives_an_error() {
        let suite = toml::from_str::<TestSuite>(
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
            # query = "metta"
            selected-languages = ["pli", "en"]
        "#,
        );

        match suite {
            Err(error) => assert_eq!(error.message(), "missing field `query`"),
            Ok(_) => panic!("Did not get expected error."),
        }
    }
}
