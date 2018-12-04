use crate::{ColorValue, Hexable, Theme};
use failure::Error;
use regex::Captures;
use regex::Regex;

// TODO: Write regular expression, tests
// TODO: Think about documentation for this

pub fn mk_x_app(prefixes: [&T]) -> Box<Fn(&Theme, &str) -> Result<String, Error>> {}
