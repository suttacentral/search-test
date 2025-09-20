use crate::defaults::Defaults;
use crate::expected::{Expected, ExpectedDetails};
use crate::test_suite::DetailsProvided;
use anyhow::Context;

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
    pub fn new(defaults: &Defaults, provided: &DetailsProvided) -> anyhow::Result<TestCase> {
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

        let expected = Self::expected(&provided.expected)?;

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

    fn expected(details: &Option<ExpectedDetails>) -> anyhow::Result<Option<Expected>> {
        match details {
            Some(expected_details) => {
                let expected = Expected::try_from(expected_details)?;
                Ok(Some(expected))
            }
            None => Ok(None),
        }
    }
}
