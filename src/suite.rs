use crate::builders::SettingsBuilder;
use anyhow::{Context, Result, anyhow};
use saphyr::{LoadableYamlNode, Scalar, Yaml};
use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug)]
pub struct Settings {
    endpoint: String,
    limit: usize,
    site_language: String,
    restrict: String,
    selected_languages: Vec<String>,
    match_partial: bool,
}

impl TryFrom<&Yaml<'_>> for Settings {
    type Error = anyhow::Error;

    fn try_from(yaml: &Yaml) -> Result<Self> {
        if let Yaml::Mapping(settings) = &yaml["settings"] {
            let endpoint_key = &Yaml::Value(Scalar::String(Cow::from("endpoint")));

            let endpoint = settings
                .get(endpoint_key)
                .context("Missing endpoint")?
                .as_str()
                .context("Endpoint not a string")?
                .to_string();

            Ok(Settings {
                endpoint,
                limit: 50,
                site_language: "en".to_string(),
                restrict: "all".to_string(),
                selected_languages: vec!["en".to_string()],
                match_partial: false,
            })
        } else {
            Err(anyhow!("Settings is not a mapping"))
        }
    }
}

#[derive(Debug)]
pub struct TestCase {
    description: Option<String>,
    query: String,
    selected_languages: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct TestSuite {
    settings: Settings,
    test_cases: Vec<TestCase>,
}

impl TryFrom<&str> for TestSuite {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        let docs = Yaml::load_from_str(value).context("Could not parse YAML")?;
        let settings = Settings::try_from(&docs[0]).context("Error getting settings.")?;
        let test_cases = vec![TestCase {
            description: Some("Search for the metta sutta in English and Pali".to_string()),
            query: "metta".to_string(),
            selected_languages: None,
        }];

        Ok(TestSuite {
            settings,
            test_cases,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_endpoint() {
        let yaml = SettingsBuilder::new().endpoint("http://abc").yaml_text();
        let suite = TestSuite::try_from(yaml.as_str());
        assert_eq!(suite.unwrap().settings.endpoint, "http://abc");
    }

    #[test]
    fn missing_endpoint_is_error() {
        let yaml = SettingsBuilder::new().limit(50).yaml_text();
        let suite = TestSuite::try_from(yaml.as_str());
        assert!(suite.is_err())
    }

    #[test]
    fn can_parse_settings() {
        let yaml = SettingsBuilder::new()
            .endpoint("http://localhost/api/search/instant")
            .limit(50)
            .site_language("en")
            .restrict("all")
            .selected_languages(vec![
                "lzh", "en", "pgd", "kho", "pli", "pra", "san", "xct", "xto", "uig",
            ])
            .match_partial(true)
            .yaml_text();

        let docs = Yaml::load_from_str(yaml.as_str()).unwrap();
        assert_eq!(docs.len(), 1);
        let settings = &docs[0];
    }

    #[test]
    fn import_suite_settings() {
        let yaml = SettingsBuilder::new()
            .endpoint("http://localhost/api/search/instant")
            .limit(50)
            .site_language("en")
            .restrict("all")
            .selected_languages(vec!["en", "pli"])
            .match_partial(true)
            .yaml_text();

        let suite = TestSuite::try_from(yaml.as_str()).unwrap();
        assert_eq!(
            suite.settings.endpoint,
            "http://localhost/api/search/instant"
        );
    }
}
