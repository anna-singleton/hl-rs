use regex::Regex;

pub struct HighlightScheme {
    pub re: Regex,
    color: String,
}

impl HighlightScheme {
    pub fn new(regex_expr: &str, color: &str) -> Result<Self, regex::Error> {
        let re = regex::Regex::new(regex_expr)?;
        return Ok(HighlightScheme {re, color: color.to_string()});
    }
}
