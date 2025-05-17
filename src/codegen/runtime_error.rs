use crate::common::error_context::{ErrorContext, PrettyError};
use std::fmt;

#[derive(Clone, Debug)]
pub struct RuntimeError {
    pub message: String,
    pub context: ErrorContext,
}

impl PrettyError for RuntimeError {
    fn message(&self) -> &str {
        &self.message
    }

    fn context(&self) -> &ErrorContext {
        &self.context
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pretty_fmt(f)
    }
}
