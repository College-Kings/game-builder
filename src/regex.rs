use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref VERSION_REGEX: Regex = Regex::new(r#"define config\.version = "(.*)""#).unwrap();
}
