use std::io::{BufRead, BufReader, BufWriter, Read, Write};

use crate::highlight_scheme::HighlightScheme;

/// A struct responsible for reading from and writing to a stream,
/// doing some processing on the text flowing between them to highlight
/// it according to the given schemes.
pub struct Highlighter {
    /// Schemes to use for formatting the stream data.
    schemes: Vec<HighlightScheme>,
    /// a BufReader for the input stream. Can be on any type that implements Read
    /// but is usually stdin or a file.
    in_stream_reader: BufReader<Box<dyn Read>>,
    /// a BufWriter for the output stream. Can be on any type that implements Write
    /// but is usually stdout or a file.
    out_stream_writer: BufWriter<Box<dyn Write>>,
}

impl Highlighter {
    pub fn new(schemes: Vec<HighlightScheme>, in_stream: Box<dyn Read>, out_stream: Box<dyn Write>) -> Self {
        let in_stream_reader = BufReader::new(in_stream);
        let out_stream_writer = BufWriter::new(out_stream);
        return Highlighter { schemes, in_stream_reader, out_stream_writer };
    }

    /// read in a single line from the stream and chop any trailing newlines
    fn read_line(&mut self) -> Result<Option<String>, std::io::Error> {
        let mut line = String::new();
        let bytes_read = self.in_stream_reader.read_line(&mut line)?;
        if bytes_read == 0 { return Ok(None) }
        return Ok(Some(line.trim_end_matches('\n').to_string()));
    }

    /// apply scheme formatting on a single line
    fn process_line(&self, mut line: String) -> String {
        for scheme in self.schemes.iter() {
            line = scheme.format_line(&line);
        }
        return line;
    }

    /// write a line to the output stream.
    fn write_line(&mut self, line: String) -> Result<(), std::io::Error> {
        write!(self.out_stream_writer, "{}\n", &line)?;
        return Ok(());
    }

    /// process the input stream line-by-line until EOF is reached or an error
    /// occurs.
    pub fn process_to_eof(&mut self) -> Result<(), std::io::Error> {
        while let Some(line_in) = self.read_line()? {
            self.write_line(self.process_line(line_in))?;
        }
        // flush the stream just in case
        self.out_stream_writer.flush()?;
        return Ok(());
    }
}

impl Drop for Highlighter {
    fn drop(&mut self) {
        // when we drop our highlighter we want to be sure we actually wrote
        // everything we finished processing to the output stream.
        self.out_stream_writer.flush().expect("Could not flush output stream whilst cleaning up.");
    }
}

#[cfg(test)]
mod tests {
    use colored::{Colorize, Styles};
    use tempfile::tempfile;

    use super::*;
    use std::io::{empty, read_to_string, Seek};

    fn process_buffer(schemes: Vec<HighlightScheme>, in_buf: &str) -> String {
        let h = Highlighter::new(schemes, Box::new(empty()), Box::new(empty()));
        return in_buf
            .lines()
            .map(|line| h.process_line(line.to_string()))
        .fold(String::new(), |acc, x| acc + "\n" + &x).trim_start().to_string();
    }

    fn assert_proccessed_strings_or_report(expected_output: String, output: String) {
        if expected_output == output {
            return;
        }

        println!("encountered inequality between expected output and actual output:");
        println!("RAW DATA:");
        println!("expected output: {:?}", expected_output);
        println!("actual output: {:?}", output);
        println!("\nCOLOURISED DATA:");
        println!("expected output: {}", expected_output);
        println!("actual output: {}", output);
        panic!("expected output did not equal actual output. see logs for details.");
    }

    #[test]
    fn process_line_no_schemes_test() {
        let schemes = vec![];
        let input = "Hello!";
        let expected_output = "Hello!";

        let output = process_buffer(schemes, input);

        assert_eq!(expected_output, output);
    }

    #[test]
    fn process_line_single_highlight_test() {
        let schemes = vec![
            HighlightScheme::new(r"\[ERROR\].*", "red", Styles::Clear).expect("could not setup highlight scheme"),
        ];

        let input = "[ERROR] you need to realign the positronic matrix buffers";
        let expected_output = "[ERROR] you need to realign the positronic matrix buffers".red().to_string();

        let output = process_buffer(schemes, input);

        assert_proccessed_strings_or_report(expected_output, output);
    }

    #[test]
    fn process_line_no_matches_test() {
        let schemes = vec![
            HighlightScheme::new(r"\[ERROR\].*", "red", Styles::Clear).expect("could not setup highlight scheme"),
        ];

        let input = "some message that doesnt fit the match";
        let expected_output = "some message that doesnt fit the match".to_string();

        let output = process_buffer(schemes, input);

        assert_proccessed_strings_or_report(expected_output, output);
    }

    #[test]
    fn process_line_multi_line_multi_match() {
        let schemes = vec![
            HighlightScheme::new(r"\[ERROR\].*", "red", Styles::Clear).expect("could not setup highlight scheme"),
            HighlightScheme::new(r"\[WARN\].*", "yellow", Styles::Clear).expect("could not setup highlight scheme"),
        ];

        let input = "[ERROR] you need to realign the positronic matrix buffers\n[WARN] the fields array is slightly out of alignment.";
        let expected_output = format!("{}\n{}", "[ERROR] you need to realign the positronic matrix buffers".red().to_string(), &"[WARN] the fields array is slightly out of alignment.".yellow());

        let output = process_buffer(schemes, input);

        assert_proccessed_strings_or_report(expected_output, output);
    }

    #[test]
    fn streams_singleline_test() {
        let mut input_file = tempfile().unwrap();
        let output_file = tempfile().unwrap();
        let mut output_file_2 = output_file.try_clone().unwrap();

        let input = "this is my input file.";

        let expected_output = "this is my \u{1b}[32minput\u{1b}[0m \u{1b}[36mfile\u{1b}[0m.\n";

        write!(input_file, "{}", input).expect("could not write test data to tmp input file.");
        input_file.flush().expect("could not flush test data to input file");
        input_file.rewind().expect("could not rewind input file stream");

        let schemes = vec![
            HighlightScheme::new(r"input", "green", Styles::Clear).expect("could not setup highlight scheme"),
            HighlightScheme::new(r"file", "cyan", Styles::Clear).expect("could not setup highlight scheme"),
        ];

        let mut h = Highlighter::new(schemes, Box::new(input_file), Box::new(output_file));

        h.process_to_eof().expect("error during highlighter processing");

        drop(h);
        output_file_2.rewind().expect("could not rewind to beginning of stream.");
        let reader = BufReader::new(output_file_2);
        let output = read_to_string(reader).expect("could not read from output tmpfile");
        assert_proccessed_strings_or_report(expected_output.to_string(), output);
    }

    #[test]
    fn streams_multiline_test() {
        let mut input_file = tempfile().unwrap();
        let output_file = tempfile().unwrap();
        let mut output_file_2 = output_file.try_clone().unwrap();

        let input = "\
this is my input file.
this is a second line.
this is the final line.
";

        let expected_output = "\
this is my input file.
this is a \u{1b}[32msecond\u{1b}[0m line.
this is the \u{1b}[36mfinal\u{1b}[0m line.
";

        write!(input_file, "{}", input).expect("could not write test data to tmp input file.");
        input_file.flush().expect("could not flush test data to input file");
        input_file.rewind().expect("could not rewind input file stream");

        let schemes = vec![
            HighlightScheme::new(r"\[ERROR\].*", "red", Styles::Clear).expect("could not setup highlight scheme"),
            HighlightScheme::new(r"\[WARN\].*", "yellow", Styles::Clear).expect("could not setup highlight scheme"),
            HighlightScheme::new(r"second", "green", Styles::Clear).expect("could not setup highlight scheme"),
            HighlightScheme::new(r"final", "cyan", Styles::Clear).expect("could not setup highlight scheme"),
        ];

        let mut h = Highlighter::new(schemes, Box::new(input_file), Box::new(output_file));

        h.process_to_eof().expect("error during highlighter processing");

        drop(h);
        output_file_2.rewind().expect("could not rewind to beginning of stream.");
        let reader = BufReader::new(output_file_2);
        let output = read_to_string(reader).expect("could not read from output tmpfile");
        assert_proccessed_strings_or_report(expected_output.to_string(), output);
    }
}
