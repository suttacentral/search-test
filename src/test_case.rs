use crate::defaults::Defaults;
use crate::expected::{Expected, ExpectedDetails};
use crate::test_suite::TestCaseDetails;
use anyhow::{Context, Result};

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
    pub fn new(defaults: &Defaults, provided: &TestCaseDetails) -> Result<TestCase> {
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

        let expected = Self::expected(&provided.expected)
            .context(Self::expected_error_message(&description))?;

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

    fn missing_message(description: &str, key: &str) -> String {
        format!("Test case `{description}` missing `{key}` and no default provided.")
    }

    fn expected(details: &Option<ExpectedDetails>) -> Result<Option<Expected>> {
        match details {
            Some(expected_details) => {
                let expected = Expected::try_from(expected_details)?;
                Ok(Some(expected))
            }
            None => Ok(None),
        }
    }

    fn expected_error_message(description: &str) -> String {
        format!("Test case `{description}`")
    }
}

#[cfg(test)]
mod tests {
    use crate::defaults::Defaults;
    use crate::test_case::TestCase;
    use crate::test_suite::TestCaseDetails;

    fn all_details_but_expected() -> TestCaseDetails {
        TestCaseDetails {
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

    fn defaults() -> Defaults {
        Defaults {
            limit: Some(50),
            site_language: Some("en".to_string()),
            restrict: Some("all".to_string()),
            selected_languages: Some(vec!["en".to_string(), "pli".to_string()]),
            match_partial: Some(false),
        }
    }

    fn test_case() -> TestCase {
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

    #[test]
    fn construct_with_defaults() {
        let details = TestCaseDetails {
            description: "Search in English only.".to_string(),
            query: "metta".to_string(),
            site_language: Some("en".to_string()),
            selected_languages: Some(vec!["en".to_string()]),
            match_partial: Some(false),
            limit: None,
            restrict: None,
            expected: None,
        };

        let actual = TestCase::new(&defaults(), &details).unwrap();

        assert_eq!(actual, test_case());
    }

    #[test]
    fn construct_without_defaults() {
        let actual = TestCase::new(&Defaults::default(), &all_details_but_expected()).unwrap();
        assert_eq!(actual, test_case());
    }

    #[test]
    fn error_when_both_are_missing_site_language() {
        let defaults = Defaults {
            site_language: None,
            ..defaults()
        };

        let details = TestCaseDetails {
            description: "Search in English only.".to_string(),
            query: "metta".to_string(),
            site_language: None,
            selected_languages: Some(vec!["en".to_string()]),
            match_partial: Some(false),
            limit: Some(50),
            restrict: Some("all".to_string()),
            expected: None,
        };

        if let Err(error) = TestCase::new(&defaults, &details) {
            assert_eq!(
                error.to_string(),
                "Test case `Search in English only.` missing `site-language` and no default provided."
            );
        }
    }

    #[test]
    fn error_when_site_language_missing() {
        let missing = TestCaseDetails {
            site_language: None,
            ..all_details_but_expected()
        };

        let error = TestCase::new(&Defaults::default(), &missing).unwrap_err();

        assert_eq!(
            error.to_string(),
            "Test case `Search in English only.` missing `site-language` and no default provided."
        );
    }

    #[test]
    fn error_when_selected_languages_missing() {
        let missing = TestCaseDetails {
            selected_languages: None,
            ..all_details_but_expected()
        };

        let error = TestCase::new(&Defaults::default(), &missing).unwrap_err();

        assert_eq!(
            error.to_string(),
            "Test case `Search in English only.` missing `selected-languages` and no default provided."
        );
    }

    #[test]
    fn error_when_match_partial_missing() {
        let missing = TestCaseDetails {
            match_partial: None,
            ..all_details_but_expected()
        };

        let error = TestCase::new(&Defaults::default(), &missing).unwrap_err();

        assert_eq!(
            error.to_string(),
            "Test case `Search in English only.` missing `match-partial` and no default provided."
        );
    }

    #[test]
    fn error_when_limit_missing() {
        let missing = TestCaseDetails {
            limit: None,
            ..all_details_but_expected()
        };

        let error = TestCase::new(&Defaults::default(), &missing).unwrap_err();

        assert_eq!(
            error.to_string(),
            "Test case `Search in English only.` missing `limit` and no default provided."
        );
    }

    #[test]
    fn error_when_restrict_missing() {
        let missing = TestCaseDetails {
            restrict: None,
            ..all_details_but_expected()
        };

        let error = TestCase::new(&Defaults::default(), &missing).unwrap_err();

        assert_eq!(
            error.to_string(),
            "Test case `Search in English only.` missing `restrict` and no default provided."
        );
    }
}
