mod cli;

use std::process::exit;
use cli::Cli;
use hl2_lib::highlighter::Highlighter;

fn main() {
    let try_schemes = Cli::create_schemes_from_cli();
    let Ok(schemes) = try_schemes else {
        eprintln!("Error building highlight schemes from regex.");
        let e = try_schemes.unwrap_err();
        eprintln!("Error: {:?}", e);
        exit(1);
    };

    let mut h = Highlighter::new(
        schemes,
        Box::new(std::io::stdin()),
        Box::new(std::io::stdout()));
    if let Err(e) = h.process_to_eof() {
        panic!("Error during stream processing. {}", e);
    }
}
