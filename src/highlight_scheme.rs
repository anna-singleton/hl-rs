use std::str::FromStr;

use colored::{Color, Styles, Colorize};
use regex::Regex;

pub struct HighlightScheme {
    /// regex for matching what data needs formatting
    re: Regex,
    /// the colour, if any, to format with.
    color: Color,
    /// the style, if any, to format with.
    style: Styles,
}

impl HighlightScheme {
    pub fn new(regex_expr: &str, color: &str, style: Styles) -> Result<Self, regex::Error> {
        let re = regex::Regex::new(regex_expr)?;
        return Ok(HighlightScheme {re, color: Color::from_str(color).expect("could not get color"), style });
    }

    pub fn builder(regex_expr: &str) -> Result<HighlightSchemeBuilder, regex::Error> {
        let re = regex::Regex::new(regex_expr)?;
        return Ok(HighlightSchemeBuilder { re, color: None, style: None });
    }

    /// This function formats the whole slice passed to it, it does not re-check the regex.
    pub fn format_whole_slice(&self, string_to_format: &str) -> String {
        let colored_string = string_to_format.to_string().color(self.color);
        return match self.style {
            Styles::Clear => colored_string,
            Styles::Bold => colored_string.bold(),
            Styles::Dimmed => colored_string.dimmed(),
            Styles::Underline => colored_string.underline(),
            Styles::Reversed => colored_string.reversed(),
            Styles::Italic => colored_string.italic(),
            Styles::Blink => colored_string.blink(),
            Styles::Hidden => colored_string.hidden(),
            Styles::Strikethrough => colored_string.strikethrough(),
        }.to_string();
    }

    /// this function does the regex search and formats the correct parts of the
    /// line and returns them as a new string.
    ///
    /// N.B. this function might behave oddly if passed slices of multiple lines.
    pub fn format_line(&self, mut line_to_format: &str) -> String {
        debug_assert!(!line_to_format.contains("\n"), "format_line should not be passed slices containing \\n");
        let mut new_line = String::new();
        while let Some(m) = self.re.find(&line_to_format) {
            // doing this as a push_str breaks the colour formatter,
            // so do it like this.
            new_line = format!("{}{}{}",
                new_line,
                &line_to_format[..m.start()],
                &line_to_format[m.range()].color(self.color.clone()));
            line_to_format = &line_to_format[m.end()..];
        }
        return format!("{}{}", new_line, &line_to_format);
    }
}

pub struct HighlightSchemeBuilder {
    re: Regex,
    color: Option<Color>,
    style: Option<Styles>
}

impl HighlightSchemeBuilder {
    pub fn build(self) -> HighlightScheme {
        return HighlightScheme { re: self.re, color: self.color.unwrap_or(Color::White), style: self.style.unwrap_or(Styles::Clear) };
    }

    pub fn with_style(mut self, s: Styles) -> Self {
        self.style = Some(s);
        return self;
    }

    pub fn with_color(mut self, s: Color) -> Self {
        self.color = Some(s);
        return self;
    }
}
