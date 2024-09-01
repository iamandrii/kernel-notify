use std::sync::Arc;

use super::ParserFn;
use crate::error::Error;

pub trait Parser {
    fn to_parser_fn(self) -> Arc<ParserFn>;
}

pub struct RegexParser {
    regex: regex::Regex,
    group: usize,
}

impl RegexParser {
    pub fn new(regex: &str, group: usize) -> Self {
        Self {
            regex: regex::Regex::new(regex).unwrap(),
            group,
        }
    }

    pub fn parse(&self, body: &str) -> Result<String, Error> {
        let caps = self.regex.captures(body);
        if let Some(caps) = caps {
            Ok(caps
                .get(self.group)
                .ok_or(Error::ParsingError("Unable to parse response".into()))?
                .as_str()
                .into())
        } else {
            Err(Error::GenericError("Unable to parse".into()))
        }
    }
}

impl Parser for RegexParser {
    fn to_parser_fn(self) -> Arc<ParserFn> {
        Arc::new(move |result| self.parse(&result.body))
    }
}
