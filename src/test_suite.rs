use crate::defaults::Defaults;
use crate::expected::ExpectedDetails;
use crate::test_case::TestCase;
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
struct Settings {
    endpoint: String,
    #[serde(default)]
    delay: u64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct TestCaseDetails {
    pub query: String,
    pub description: String,
    pub limit: Option<usize>,
    pub site_language: Option<String>,
    pub restrict: Option<String>,
    pub selected_languages: Option<Vec<String>>,
    pub match_partial: Option<bool>,
    pub expected: Option<ExpectedDetails>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct TestSuite {
    settings: Settings,
    #[serde(default)]
    defaults: Defaults,
    #[serde[rename = "test-case"]]
    test_details: Vec<TestCaseDetails>,
}

impl TestSuite {
    pub fn load_from_string(source: &str) -> Result<TestSuite> {
        toml::from_str::<TestSuite>(source).context("Failed to parse TOML.")
    }

    pub fn endpoint(&self) -> String {
        self.settings.endpoint.clone()
    }

    pub fn delay(&self) -> u64 {
        self.settings.delay
    }

    pub fn test_cases(&self) -> impl Iterator<Item = Result<TestCase>> {
        self.test_details
            .iter()
            .map(|details| TestCase::new(&self.defaults, details))
    }

    pub fn headline(&self) -> String {
        format!(
            "Running tests against endpoint {} with {}ms delay",
            self.endpoint(),
            self.delay()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expected::Expected;
    use crate::identifiers::{SearchResultKey, SuttaplexUid};

    #[test]
    fn can_parse_specification() {
        let suite = TestSuite::load_from_string(
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
            test_details: vec![TestCaseDetails {
                description: "Search for the metta sutta in English and Pali".to_string(),
                query: "metta".to_string(),
                selected_languages: Some(vec!["pli".to_string(), "en".to_string()]),
                limit: None,
                site_language: None,
                restrict: None,
                match_partial: None,
                expected: None,
            }],
        };

        assert_eq!(suite, expected);
    }

    #[test]
    fn error_when_not_valid_toml() {
        let error = TestSuite::load_from_string("This is not TOML").unwrap_err();
        assert_eq!(error.to_string(), "Failed to parse TOML.")
    }

    #[test]
    fn defaults_are_all_none_if_table_missing() {
        let suite = TestSuite::load_from_string(
            r#"
            [settings]
            endpoint = "http://localhost/api/search/instant"
            delay = 3000

            [[test-case]]
            description = "Search for the metta sutta in English and Pali"
            query = "metta"
            limit = 50
            site-language = "en"
            restrict = "all"
            selected-languages = ["en", "pli"]
            match-partial = false
        "#,
        )
        .unwrap();

        assert_eq!(suite.defaults, Defaults::default());
    }

    #[test]
    fn missing_description_is_parse_error() {
        let error = TestSuite::load_from_string(
            r#"
            [settings]
            endpoint = "http://localhost/api/search/instant"
            delay = 3000

            [[test-case]]
            query = "metta"
            limit = 50
            site-language = "en"
            restrict = "all"
            selected-languages = ["en", "pli"]
            match-partial = false
        "#,
        )
        .unwrap_err();
        assert_eq!(error.to_string(), "Failed to parse TOML.");
    }

    #[test]
    fn missing_query_is_parse_error() {
        let error = TestSuite::load_from_string(
            r#"
            [settings]
            endpoint = "http://localhost/api/search/instant"
            delay = 3000

            [[test-case]]
            description = "Search for the metta sutta in English and Pali"
            limit = 50
            site-language = "en"
            restrict = "all"
            selected-languages = ["en", "pli"]
            match-partial = false
        "#,
        )
        .unwrap_err();
        assert_eq!(error.to_string(), "Failed to parse TOML.");
    }

    #[test]
    fn suite_generates_test_cases() {
        let suite = TestSuite::load_from_string(
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

            [[test-case]]
            description = "Has multiple dictionary results."
            query = "pacch"
            match-partial = true
        "#,
        )
        .unwrap();

        let queries: Vec<_> = suite
            .test_cases()
            .map(|test_case| test_case.unwrap().query)
            .collect();

        assert_eq!(queries, vec!("metta", "pacch"));
    }

    #[test]
    fn suite_provides_global_settings() {
        let suite = TestSuite::load_from_string(
            r#"
            [settings]
            endpoint = "http://localhost/api/search/instant"
            delay = 3000

            [[test-case]]
            description = "Search for the metta sutta in English and Pali"
            query = "metta"
        "#,
        )
        .unwrap();

        assert_eq!(suite.endpoint(), "http://localhost/api/search/instant");
        assert_eq!(suite.delay(), 3000);
    }

    #[test]
    fn missing_delay_defaults_to_zero() {
        let suite = TestSuite::load_from_string(
            r#"
            [settings]
            endpoint = "http://localhost/api/search/instant"

            [[test-case]]
            description = "Search for the metta sutta in English and Pali"
            query = "metta"
        "#,
        )
        .unwrap();

        assert_eq!(suite.delay(), 0);
    }

    #[test]
    fn expected_provided() {
        let suite = TestSuite::load_from_string(
            r#"
            [settings]
            endpoint = "http://localhost/api/search/instant"

            [defaults]
            limit = 50
            site-language = "en"
            restrict = "all"
            selected-languages = ["en", "pli"]
            match-partial = false

            [[test-case]]
            description = "Find a suttaplex"
            query = "mn1"
            expected.suttaplex = "mn1"
            expected.min-rank = 3
        "#,
        )
        .unwrap();

        let test_case = suite.test_cases().next().unwrap().unwrap();
        let expected: Expected = test_case.expected.unwrap();
        assert_eq!(
            expected,
            Expected::Ranked {
                key: SearchResultKey::Suttaplex {
                    uid: SuttaplexUid::from("mn1")
                },
                min_rank: 3
            }
        );
    }

    #[test]
    fn expected_not_provided() {
        let suite = TestSuite::load_from_string(
            r#"
            [settings]
            endpoint = "http://localhost/api/search/instant"

            [defaults]
            limit = 50
            site-language = "en"
            restrict = "all"
            selected-languages = ["en", "pli"]
            match-partial = false

            [[test-case]]
            description = "Find a suttaplex"
            query = "mn1"
        "#,
        )
        .unwrap();

        let test_case = suite.test_cases().next().unwrap().unwrap();
        let expected = test_case.expected;
        assert!(expected.is_none());
    }

    #[test]
    fn format_headline_with_delay() {
        let suite = TestSuite::load_from_string(
            r#"
            [settings]
            endpoint = "http://localhost/api/search/instant"
            delay = 1000

            [defaults]
            limit = 50
            site-language = "en"
            restrict = "all"
            selected-languages = ["en", "pli"]
            match-partial = false

            [[test-case]]
            description = "Find a sutta"
            query = "metta"

            [[test-case]]
            description = "Find another sutta"
            query = "dosa"
        "#,
        )
        .unwrap();

        assert_eq!(
            suite.headline(),
            "Running tests against endpoint http://localhost/api/search/instant with 1000ms delay"
        )
    }

    #[test]
    fn two_expected_type_gives_meaningful_error_message() {
        let suite = TestSuite::load_from_string(
            r#"
            [settings]
            endpoint = "http://localhost/api/search/instant"
            delay = 1000

            [defaults]
            limit = 50
            site-language = "en"
            restrict = "all"
            selected-languages = ["en", "pli"]
            match-partial = false

            [[test-case]]
            description = "Has two types expected"
            query = "metta"
            expected.sutta = "/snp5.1/en/sujato"
            expected.dictionary = "/define/metta"
        "#,
        )
        .unwrap();

        let error = suite.test_cases().next().unwrap().unwrap_err();

        assert_eq!(
            format!("{error:#}"),
            "Test case `Has two types expected`: more than one expected result provided"
        );
    }
}
