use colored::Styles;
use hl_rs::{highlight_scheme::HighlightScheme, highlighter::Highlighter};

fn main() {
    let mut h = Highlighter::new(
        vec![HighlightScheme::new(r"myerror", "red", Styles::Clear).expect("couldnt parse regex")],
        Box::new(std::io::stdin()),
        Box::new(std::io::stdout()));
    if let Err(e) = h.process_to_eof() {
        panic!("Error during stream processing. {}", e);
    }
}
