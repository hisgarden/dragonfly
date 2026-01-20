//! Color and formatting utilities

use colored::*;

pub fn success(msg: &str) -> String {
    format!("{}", msg.green().bold())
}

pub fn error(msg: &str) -> String {
    format!("{}", msg.red().bold())
}

pub fn warning(msg: &str) -> String {
    format!("{}", msg.yellow().bold())
}

pub fn info(msg: &str) -> String {
    format!("{}", msg.cyan())
}

pub fn highlight(msg: &str) -> String {
    format!("{}", msg.bright_cyan().bold())
}

pub fn dimmed(msg: &str) -> String {
    format!("{}", msg.dimmed())
}
