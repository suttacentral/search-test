use std::fmt::Write as _;

#[derive(Default)]
pub struct SettingsBuilder {
    endpoint: Option<String>,
    limit: Option<usize>,
    site_language: Option<String>,
    restrict: Option<String>,
    selected_languages: Option<String>,
    match_partial: Option<bool>,
}

impl SettingsBuilder {
    pub fn new() -> SettingsBuilder {
        SettingsBuilder {
            endpoint: None,
            limit: None,
            site_language: None,
            restrict: None,
            selected_languages: None,
            match_partial: None,
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

    pub fn selected_languages(mut self, selected_languages: Vec<&str>) -> SettingsBuilder {
        let quoted_with_commas = selected_languages
            .into_iter()
            .map(|s| format!("\"{s}\""))
            .collect::<Vec<String>>()
            .join(", ");

        let bracketed = format!("[ {quoted_with_commas} ]");

        self.selected_languages = Some(bracketed);
        self
    }

    pub fn match_partial(mut self, match_partial: bool) -> SettingsBuilder {
        self.match_partial = Some(match_partial);
        self
    }

    pub fn yaml_text(self) -> String {
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
        if let Some(selected_languages) = self.selected_languages {
            writeln!(&mut output, "    selected-languages: {selected_languages}").unwrap();
        }
        if let Some(match_partial) = self.match_partial {
            writeln!(&mut output, "    match-partial: {match_partial}").unwrap();
        }
        output
    }
}
