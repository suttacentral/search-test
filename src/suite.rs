use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
struct Settings {
    endpoint: String,
    delay: usize,
}

#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
struct Defaults {
    limit: Option<usize>,
    site_language: Option<String>,
    restrict: Option<String>,
    selected_languages: Option<Vec<String>>,
    match_partial: Option<bool>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
struct DetailsProvided {
    query: Option<String>,
    description: Option<String>,
    limit: Option<usize>,
    site_language: Option<String>,
    restrict: Option<String>,
    selected_languages: Option<Vec<String>>,
    match_partial: Option<bool>,
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
    fn combine(defaults: &Defaults, provided: &DetailsProvided) -> Result<TestCase> {
        let description = provided
            .description
            .clone()
            .context("Test case is missing description")?;

        let query = provided
            .query
            .clone()
            .context("Test case is missing query")?;

        let site_language = [&provided.site_language, &defaults.site_language]
            .into_iter()
            .find(|x| x.is_some())
            .context("Test case missing site-language and no default provided.")?
            .clone()
            .unwrap();

        let selected_languages = [&provided.selected_languages, &defaults.selected_languages]
            .into_iter()
            .find(|x| x.is_some())
            .context("Test case missing selected-languages and no default provided.")?
            .clone()
            .unwrap();

        let match_partial = [&provided.match_partial, &defaults.match_partial]
            .into_iter()
            .find(|x| x.is_some())
            .context("Test case missing match-partial and no default provided.")?
            .unwrap();

        let limit = [&provided.limit, &defaults.limit]
            .into_iter()
            .find(|x| x.is_some())
            .context("Test case missing limit and no default provided.")?
            .unwrap();

        let restrict = [&provided.restrict, &defaults.restrict]
            .into_iter()
            .find(|x| x.is_some())
            .context("Test case missing restrict and no default provided.")?
            .clone()
            .unwrap();

        Ok(TestCase {
            description,
            query,
            site_language,
            selected_languages,
            match_partial,
            limit,
            restrict,
        })
    }
}

impl TestSuite {
    pub fn load_from_string(source: &str) -> Result<TestSuite> {
        toml::from_str(source).context("Failed to parse TOML.")
    }

    pub fn endpoint(&self) -> String {
        self.settings.endpoint.clone()
    }

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

    fn complete_details() -> DetailsProvided {
        DetailsProvided {
            description: Some("Search in English only.".to_string()),
            query: Some("metta".to_string()),
            site_language: Some("en".to_string()),
            selected_languages: Some(vec!["en".to_string()]),
            match_partial: Some(false),
            limit: Some(50),
            restrict: Some("all".to_string()),
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
    fn error_when_not_valid_toml() {
        let error = TestSuite::load_from_string("This is not TOML").unwrap_err();
        assert_eq!(error.to_string(), "Failed to parse TOML.")
    }

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
        let test_case = TestCase::combine(&defaults, &complete_details()).unwrap();
        assert_eq!(test_case, example_test_case());
    }

    #[test]
    fn error_when_both_are_missing_site_language() {
        let defaults = Defaults {
            site_language: None,
            ..example_defaults()
        };

        let details = DetailsProvided {
            description: Some("Search in English only.".to_string()),
            query: Some("metta".to_string()),
            site_language: None,
            selected_languages: Some(vec!["en".to_string()]),
            match_partial: Some(false),
            limit: Some(50),
            restrict: Some("all".to_string()),
        };

        if let Err(error) = TestCase::combine(&defaults, &details) {
            assert_eq!(
                error.to_string(),
                "Test case missing site-language and no default provided."
            );
        }
    }

    #[test]
    fn combine_gives_error_when_site_language_missing() {
        let missing = DetailsProvided {
            site_language: None,
            ..complete_details()
        };

        let error = TestCase::combine(&Defaults::default(), &missing).unwrap_err();

        assert_eq!(
            error.to_string(),
            "Test case missing site-language and no default provided."
        );
    }

    #[test]
    fn combine_gives_error_when_selected_languages_missing() {
        let missing = DetailsProvided {
            selected_languages: None,
            ..complete_details()
        };

        let error = TestCase::combine(&Defaults::default(), &missing).unwrap_err();

        assert_eq!(
            error.to_string(),
            "Test case missing selected-languages and no default provided."
        );
    }

    #[test]
    fn combine_gives_error_when_match_partial_missing() {
        let missing = DetailsProvided {
            match_partial: None,
            ..complete_details()
        };

        let error = TestCase::combine(&Defaults::default(), &missing).unwrap_err();

        assert_eq!(
            error.to_string(),
            "Test case missing match-partial and no default provided."
        );
    }

    #[test]
    fn combine_gives_error_when_limit_missing() {
        let missing = DetailsProvided {
            limit: None,
            ..complete_details()
        };

        let error = TestCase::combine(&Defaults::default(), &missing).unwrap_err();

        assert_eq!(
            error.to_string(),
            "Test case missing limit and no default provided."
        );
    }

    #[test]
    fn combine_gives_error_when_restrict_missing() {
        let missing = DetailsProvided {
            restrict: None,
            ..complete_details()
        };

        let error = TestCase::combine(&Defaults::default(), &missing).unwrap_err();

        assert_eq!(
            error.to_string(),
            "Test case missing restrict and no default provided."
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
}
