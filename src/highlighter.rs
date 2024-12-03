use std::io::{BufRead, BufReader, BufWriter, Read, Write};

use colored::Colorize;

use crate::highlight_scheme::HighlightScheme;

pub struct Highlighter {
    schemes: Vec<HighlightScheme>,
    in_stream: Box<dyn Read>,
    out_stream: Box<dyn Write>,
}

impl Highlighter {
    pub fn new(schemes: Vec<HighlightScheme>, in_stream: Box<dyn Read>, out_stream: Box<dyn Write>) -> Self {
        return Highlighter { schemes, in_stream, out_stream };
    }

    pub fn read_line(&mut self) -> Result<Option<String>, std::io::Error> {
        let mut line = String::new();
        let mut reader = BufReader::new(&mut self.in_stream);
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 { return Ok(None) }
        return Ok(Some(line));
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
                // doing this as a push_str breaks the colour formatter,
                // so do it like this.
                new_line = format!("{}{}{}",
                    new_line,
                    &line[..m.start()],
                    &line[m.range()].color("red"));
                line = line[m.end()..].to_string();
            }
            new_line = format!("{}{}", new_line, &line);
            line = new_line;
        }
        return line;
    }

    pub fn process_to_eof(&mut self) -> Result<(), std::io::Error> {
        while let Some(line_in) = self.read_line()? {
            self.write_line(self.process_line(line_in))?;
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{empty, Cursor};

    fn assert_streams_after_processing(schemes: Vec<HighlightScheme>, in_stream: String, expected_out: Vec<u8>) {
        let mut in_stream = Cursor::new(in_stream);
        let mut out_stream = Cursor::new(vec![]);

        let mut h = Highlighter::new(schemes, Box::new(in_stream), Box::new(out_stream));
        if let Err(e) = h.process_to_eof() {
            panic!("Error during processing. {}", e);
        }
        // dbg!(h,out_stream.get_ref());
    }

    #[test]
    fn process_line_doesnt_change_content() {
        let h = Highlighter::new(vec![], Box::new(empty()), Box::new(empty()));
        assert_eq!("hello", h.process_line("hello".to_string()));
    }
}
