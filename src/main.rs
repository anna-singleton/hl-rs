use std::io::empty;

use hl_rs::{Highlighter, HighlightScheme};
use regex::Regex;

fn main() {
    let h = Highlighter {
        schemes: vec![HighlightScheme {
            re: Regex::new(r"myerror").unwrap(),
            col: "red".to_string()}
        ],
        in_stream: Box::new(empty()),
        out_stream: Box::new(empty())};

    let s = h.process_line("something myerror somethingelse".to_string());
    println!("{}", s);
}
