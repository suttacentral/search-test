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
        Self(String::from(value))
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct SuttaplexUid(String);

impl Display for SuttaplexUid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for SuttaplexUid {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}
