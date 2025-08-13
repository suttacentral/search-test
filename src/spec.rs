use std::fmt::Write as _;

#[derive(Default)]
pub struct SettingsBuilder {
    endpoint: Option<String>,
    limit: Option<usize>,
    site_language: Option<String>,
    restrict: Option<String>,
}

impl SettingsBuilder {
    pub fn new() -> SettingsBuilder {
        SettingsBuilder {
            endpoint: None,
            limit: None,
            site_language: None,
            restrict: None,
        }
    }

    pub fn endpoint(mut self, endpoint: &str) -> SettingsBuilder {
        self.endpoint = Some(String::from(endpoint));
        self
    }

    pub fn limit(mut self, limit: usize) -> SettingsBuilder {
        self.limit = Some(limit);
        self
    }

    pub fn site_language(mut self, site_language: &str) -> SettingsBuilder {
        self.site_language = Some(String::from(site_language));
        self
    }

    pub fn restrict(mut self, restrict: &str) -> SettingsBuilder {
        self.restrict = Some(String::from(restrict));
        self
    }

    pub fn build(self) -> String {
        let mut output = String::new();
        writeln!(&mut output, "settings: ").unwrap();
        if let Some(endpoint) = self.endpoint {
            writeln!(&mut output, "    endpoint: \"{endpoint}\"").unwrap();
        }
        if let Some(limit) = self.limit {
            writeln!(&mut output, "    limit: {limit}").unwrap();
        }
        if let Some(site_language) = self.site_language {
            writeln!(&mut output, "    site-language: \"{site_language}\"").unwrap();
        }
        if let Some(restrict) = self.restrict {
            writeln!(&mut output, "    restrict: \"{restrict}\"").unwrap();
        }
        output
    }
}
