use std::io::{BufRead, BufReader, BufWriter, Read, Write};

use colored::Colorize;
use regex::Regex;

pub struct HighlightScheme {
    pub re: Regex,
    pub col: String,
}

pub struct Highlighter {
    pub schemes: Vec<HighlightScheme>,
    pub in_stream: Box<dyn Read>,
    pub out_stream: Box<dyn Write>,
}

impl Highlighter {
    pub fn read_line(&mut self) -> Result<String, std::io::Error> {
        let mut line = String::new();
        let mut reader = BufReader::new(&mut self.in_stream);
        reader.read_line(&mut line)?;
        return Ok(line);
    }

    pub fn write_line(&mut self, line: String) -> Result<(), std::io::Error> {
        let mut writer = BufWriter::new(&mut self.out_stream);
        write!(writer, "{}", &line)?;
        return Ok(());
    }

    pub fn process_line(&self, mut line: String) -> String {
        for scheme in self.schemes.iter() {
            let mut new_line = String::new();
            while let Some(m) = scheme.re.find(&line) {
                new_line = format!("{}{}{}", new_line, &line[..m.start()-1], &line[m.range()].color("red"));
                line = dbg!(line[m.end()+1..].to_string());
            }
            new_line = format!("{}{}", new_line, &line);
            line = new_line;
        }
        return line;
    }
}
