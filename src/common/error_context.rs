use crate::common::source_map::get_source_map;
use colored::Colorize;
use std::fmt;

#[derive(Clone, Debug)]
pub struct ErrorContext {
    pub line_number: usize,
    pub lexeme: String,
}

pub trait PrettyError: fmt::Display {
    fn message(&self) -> &str;
    fn context(&self) -> &ErrorContext;

    fn pretty_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ctx = self.context();
        let line = get_source_map()
            .and_then(|m| m.get_line(ctx.line_number))
            .unwrap_or("<source line unavailable>");

        let first_lexeme_line = ctx.lexeme.lines().next().unwrap_or("");

        let column_start = line.find(first_lexeme_line).unwrap_or(0);
        let column_end = column_start + first_lexeme_line.len().max(1);

        let line_prefix = format!("{:>4} | ", ctx.line_number).bright_blue().bold();

        let underline = format!(
            "{}{} {}",
            " ".repeat(line_prefix.len() + column_start),
            "^".repeat(column_end - column_start).bright_red().bold(),
            self.message().bright_red().bold()
        );

        writeln!(f, "{}{}", line_prefix, line)?;
        writeln!(f, "{}", underline)?;

        Ok(())
    }
}
