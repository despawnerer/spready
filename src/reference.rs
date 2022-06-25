use std::fmt;
use std::str::FromStr;

use arrayvec::ArrayString;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct InvalidReference;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reference(ArrayString<5>);

impl FromStr for Reference {
    type Err = InvalidReference;

    fn from_str(value: &str) -> Result<Reference, InvalidReference> {
        static REFERENCE_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^[a-zA-Z]{1,2}[1-9][0-9]{0,2}$").unwrap());

        if !REFERENCE_REGEX.is_match(value) {
            return Err(InvalidReference);
        }

        let value = match ArrayString::from(value) {
            Ok(x) => x,
            Err(_) => return Err(InvalidReference),
        };

        Ok(Reference(value))
    }
}

impl<'a> From<&'a str> for Reference {
    fn from(src: &'a str) -> Reference {
        src.parse().unwrap()
    }
}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
