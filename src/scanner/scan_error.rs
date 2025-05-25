use crate::common::error_context::{ErrorContext, PrettyError};
use colored::Colorize;
use std::fmt;

#[derive(Debug)]
pub struct ScanError {
    pub message: String,
    pub context: ErrorContext,
}

impl PrettyError for ScanError {
    fn message(&self) -> &str {
        &self.message
    }

    fn context(&self) -> &ErrorContext {
        &self.context
    }
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", "[ScanError]".red().bold())?;
        self.pretty_fmt(f)
    }
}
