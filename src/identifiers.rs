use serde::Deserialize;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct SuttacentralUrl(String);

impl Display for SuttacentralUrl {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for SuttacentralUrl {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
