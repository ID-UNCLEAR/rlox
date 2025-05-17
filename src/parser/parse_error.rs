use crate::common::error_context::{ErrorContext, PrettyError};
use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub context: ErrorContext,
}

impl PrettyError for ParseError {
    fn message(&self) -> &str {
        &self.message
    }

    fn context(&self) -> &ErrorContext {
        &self.context
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pretty_fmt(f)
    }
}
