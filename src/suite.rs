use anyhow::{Context, Result, anyhow};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Settings {
    pub endpoint: String,
    pub delay: usize,
}

#[derive(Debug, Default, Deserialize, PartialEq)]
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
pub struct DetailsProvided {
    pub query: Option<String>,
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
    pub defaults: Defaults, // Todo: Make optional and use default.
    #[serde[rename = "test-case"]]
    pub test_cases: Vec<DetailsProvided>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TestCase {
    pub query: String,
    pub description: String,
    pub limit: usize,
    pub site_language: String,
    pub restrict: String,
    pub selected_languages: Vec<String>,
    pub match_partial: bool,
}

impl TestCase {
    fn site_language(provided: &Option<String>, default: &Option<String>) -> Result<String> {
        [provided, default]
            .into_iter()
            .find(|x| x.is_some())
            .context("Test case missing site-language and no default provided.")?
            .clone()
            .context("Uh oh, we found it but didn't find it.")
    }

    pub fn combine(defaults: &Defaults, provided: &DetailsProvided) -> Result<TestCase> {
        let description = provided
            .description
            .clone()
            .context("Test case is missing description")?;

        let query = provided
            .query
            .clone()
            .context("Test case is missing query")?;

        let site_language = Self::site_language(&provided.site_language, &defaults.site_language)?;

        Ok(TestCase {
            description,
            query,
            site_language,
            selected_languages: vec!["en".to_string()],
            match_partial: false,
            limit: 50,
            restrict: "all".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        }
    }

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
            test_cases: vec![DetailsProvided {
                description: Some("Search for the metta sutta in English and Pali".to_string()),
                query: Some("metta".to_string()),
                selected_languages: Some(vec!["pli".to_string(), "en".to_string()]),
                limit: None,
                site_language: None,
                restrict: None,
                match_partial: None,
            }],
        };

        assert_eq!(suite, expected);
    }

    #[test]
    fn can_combine_provided_details_with_defaults_to_get_test_case() {
        let details = DetailsProvided {
            description: Some("Search in English only.".to_string()),
            query: Some("metta".to_string()),
            site_language: Some("en".to_string()),
            selected_languages: Some(vec!["en".to_string()]),
            match_partial: Some(false),
            limit: None,
            restrict: None,
        };

        let test_case = TestCase::combine(&example_defaults(), &details).unwrap();

        assert_eq!(test_case, example_test_case());
    }

    #[test]
    fn can_combine_missing_defaults() {
        let defaults = Defaults::default();

        let details = DetailsProvided {
            description: Some("Search in English only.".to_string()),
            query: Some("metta".to_string()),
            site_language: Some("en".to_string()),
            selected_languages: Some(vec!["en".to_string()]),
            match_partial: Some(false),
            limit: Some(1),
            restrict: Some("all".to_string()),
        };

        let test_case = TestCase::combine(&defaults, &details).unwrap();

        assert_eq!(test_case, example_test_case());
    }

    #[test]
    fn error_when_both_are_missing_site_language() {
        let defaults = Defaults {
            site_language: None,
            ..example_defaults()
        };

        // Todo: extract example and use struct update syntax.
        let details = DetailsProvided {
            description: Some("Search in English only.".to_string()),
            query: Some("metta".to_string()),
            site_language: None,
            selected_languages: Some(vec!["en".to_string()]),
            match_partial: Some(false),
            limit: Some(1),
            restrict: Some("all".to_string()),
        };

        if let Err(error) = TestCase::combine(&defaults, &details) {
            assert_eq!(
                error.to_string(),
                "Test case missing site-language and no default provided."
            );
        }
    }
}
