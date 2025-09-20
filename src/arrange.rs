use crate::defaults::Defaults;
use crate::expected::{Expected, ExpectedDetails};
use crate::identifiers::{SuttaplexUid, TextUrl};
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
struct Settings {
    endpoint: String,
    #[serde(default)]
    delay: usize,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct SuttaHitAssertion {
    pub top: TextUrl,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct DetailsProvided {
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
    test_details: Vec<DetailsProvided>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TestCase {
    pub query: String,
    pub description: String,
    pub limit: usize,
    pub site_language: String,
    pub restrict: String,
    pub selected_languages: Vec<String>,
    pub match_partial: bool,
    pub expected: Option<Expected>,
}

impl TestCase {
    fn missing_message(description: &str, key: &str) -> String {
        format!("Test case `{description}` missing `{key}` and no default provided.")
    }

    fn combine(defaults: &Defaults, provided: &DetailsProvided) -> Result<TestCase> {
        let description = provided.description.clone();
        let query = provided.query.clone();
        let site_language = [&provided.site_language, &defaults.site_language]
            .into_iter()
            .find(|x| x.is_some())
            .context(Self::missing_message(description.as_str(), "site-language"))?
            .clone()
            .unwrap();

        let selected_languages = [&provided.selected_languages, &defaults.selected_languages]
            .into_iter()
            .find(|x| x.is_some())
            .context(Self::missing_message(
                description.as_str(),
                "selected-languages",
            ))?
            .clone()
            .unwrap();

        let match_partial = [&provided.match_partial, &defaults.match_partial]
            .into_iter()
            .find(|x| x.is_some())
            .context(Self::missing_message(description.as_str(), "match-partial"))?
            .unwrap();

        let limit = [&provided.limit, &defaults.limit]
            .into_iter()
            .find(|x| x.is_some())
            .context(Self::missing_message(description.as_str(), "limit"))?
            .unwrap();

        let restrict = [&provided.restrict, &defaults.restrict]
            .into_iter()
            .find(|x| x.is_some())
            .context(Self::missing_message(description.as_str(), "restrict"))?
            .clone()
            .unwrap();

        let mut expected = None;

        if let Some(details) = &provided.expected {
            expected = Some(Expected::try_from(details)?);
        }

        Ok(TestCase {
            description,
            query,
            site_language,
            selected_languages,
            match_partial,
            limit,
            restrict,
            expected,
        })
    }
}

impl TestSuite {
    pub fn load_from_string(source: &str) -> Result<TestSuite> {
        toml::from_str::<TestSuite>(source).context("Failed to parse TOML.")
    }

    pub fn endpoint(&self) -> String {
        self.settings.endpoint.clone()
    }

    #[allow(unused)]
    pub fn delay(&self) -> usize {
        self.settings.delay
    }

    pub fn test_cases(&self) -> Result<Vec<TestCase>> {
        self.test_details
            .iter()
            .map(|details| TestCase::combine(&self.defaults, details))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expected::Expected;
    use crate::identifiers::SearchResultKey;

    fn example_defaults() -> Defaults {
        Defaults {
            limit: Some(50),
            site_language: Some("en".to_string()),
            restrict: Some("all".to_string()),
            selected_languages: Some(vec!["en".to_string(), "pli".to_string()]),
            match_partial: Some(false),
        }
    }

    fn example_test_case() -> TestCase {
        TestCase {
            description: "Search in English only.".to_string(),
            query: "metta".to_string(),
            site_language: "en".to_string(),
            selected_languages: vec!["en".to_string()],
            match_partial: false,
            limit: 50,
            restrict: "all".to_string(),
            expected: None,
        }
    }

    fn all_details_but_expected() -> DetailsProvided {
        DetailsProvided {
            description: "Search in English only.".to_string(),
            query: "metta".to_string(),
            site_language: Some("en".to_string()),
            selected_languages: Some(vec!["en".to_string()]),
            match_partial: Some(false),
            limit: Some(50),
            restrict: Some("all".to_string()),
            expected: None,
        }
    }

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
            test_details: vec![DetailsProvided {
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
    fn can_combine_provided_details_with_defaults_to_get_test_case() {
        let details = DetailsProvided {
            description: "Search in English only.".to_string(),
            query: "metta".to_string(),
            site_language: Some("en".to_string()),
            selected_languages: Some(vec!["en".to_string()]),
            match_partial: Some(false),
            limit: None,
            restrict: None,
            expected: None,
        };

        let test_case = TestCase::combine(&example_defaults(), &details).unwrap();

        assert_eq!(test_case, example_test_case());
    }

    #[test]
    fn can_combine_missing_defaults() {
        let defaults = Defaults::default();
        let test_case = TestCase::combine(&defaults, &all_details_but_expected()).unwrap();
        assert_eq!(test_case, example_test_case());
    }

    #[test]
    fn error_when_both_are_missing_site_language() {
        let defaults = Defaults {
            site_language: None,
            ..example_defaults()
        };

        let details = DetailsProvided {
            description: "Search in English only.".to_string(),
            query: "metta".to_string(),
            site_language: None,
            selected_languages: Some(vec!["en".to_string()]),
            match_partial: Some(false),
            limit: Some(50),
            restrict: Some("all".to_string()),
            expected: None,
        };

        if let Err(error) = TestCase::combine(&defaults, &details) {
            assert_eq!(
                error.to_string(),
                "Test case `Search in English only.` missing `site-language` and no default provided."
            );
        }
    }

    #[test]
    fn combine_gives_error_when_site_language_missing() {
        let missing = DetailsProvided {
            site_language: None,
            ..all_details_but_expected()
        };

        let error = TestCase::combine(&Defaults::default(), &missing).unwrap_err();

        assert_eq!(
            error.to_string(),
            "Test case `Search in English only.` missing `site-language` and no default provided."
        );
    }

    #[test]
    fn combine_gives_error_when_selected_languages_missing() {
        let missing = DetailsProvided {
            selected_languages: None,
            ..all_details_but_expected()
        };

        let error = TestCase::combine(&Defaults::default(), &missing).unwrap_err();

        assert_eq!(
            error.to_string(),
            "Test case `Search in English only.` missing `selected-languages` and no default provided."
        );
    }

    #[test]
    fn combine_gives_error_when_match_partial_missing() {
        let missing = DetailsProvided {
            match_partial: None,
            ..all_details_but_expected()
        };

        let error = TestCase::combine(&Defaults::default(), &missing).unwrap_err();

        assert_eq!(
            error.to_string(),
            "Test case `Search in English only.` missing `match-partial` and no default provided."
        );
    }

    #[test]
    fn combine_gives_error_when_limit_missing() {
        let missing = DetailsProvided {
            limit: None,
            ..all_details_but_expected()
        };

        let error = TestCase::combine(&Defaults::default(), &missing).unwrap_err();

        assert_eq!(
            error.to_string(),
            "Test case `Search in English only.` missing `limit` and no default provided."
        );
    }

    #[test]
    fn combine_gives_error_when_restrict_missing() {
        let missing = DetailsProvided {
            restrict: None,
            ..all_details_but_expected()
        };

        let error = TestCase::combine(&Defaults::default(), &missing).unwrap_err();

        assert_eq!(
            error.to_string(),
            "Test case `Search in English only.` missing `restrict` and no default provided."
        );
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

        let metta = &suite.test_cases().unwrap()[0];
        assert_eq!(metta.query, "metta");

        let pacch = &suite.test_cases().unwrap()[1];
        assert_eq!(pacch.query, "pacch");
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
    fn test_case_gets_search_key() {
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

        let test_case = suite.test_cases().unwrap()[0].clone();
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
    fn test_expected_is_none_when_missing() {
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

        let test_case = suite.test_cases().unwrap()[0].clone();
        let expected = test_case.expected;
        assert!(expected.is_none());
    }
}
