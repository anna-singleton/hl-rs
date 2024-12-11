use clap::Parser;
use colored::Color;
use hl2_lib::highlight_scheme::HighlightScheme;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long, value_name = "REGEX_MATCHER")] // cant do short for black, as we need b/B for blue.
    black: Option<String>,

    #[arg(long="bright-black", value_name = "REGEX_MATCHER")]
    bright_black: Option<String>,

    #[arg(short, long, value_name = "REGEX_MATCHER")]
    red: Option<String>,

    #[arg(short='R', long="bright-red", value_name = "REGEX_MATCHER")]
    bright_red: Option<String>,

    #[arg(short, long, value_name = "REGEX_MATCHER")]
    green: Option<String>,

    #[arg(short='G', long="bright-green", value_name = "REGEX_MATCHER")]
    bright_green: Option<String>,

    #[arg(short, long, value_name = "REGEX_MATCHER")]
    yellow: Option<String>,

    #[arg(short='Y', long="bright-yellow", value_name = "REGEX_MATCHER")]
    bright_yellow: Option<String>,

    #[arg(short, long, value_name = "REGEX_MATCHER")]
    blue: Option<String>,

    #[arg(short='B', long="bright-blue", value_name = "REGEX_MATCHER")]
    bright_blue: Option<String>,

    #[arg(short, long, value_name = "REGEX_MATCHER")]
    magenta: Option<String>,

    #[arg(short='M', long="bright-magenta", value_name = "REGEX_MATCHER")]
    bright_magenta: Option<String>,

    #[arg(short, long, value_name = "REGEX_MATCHER")]
    cyan: Option<String>,

    #[arg(short='C', long="bright-cyan", value_name = "REGEX_MATCHER")]
    bright_cyan: Option<String>,

    #[arg(short, long, value_name = "REGEX_MATCHER")]
    white: Option<String>,

    #[arg(short='W', long="bright-white", value_name = "REGEX_MATCHER")]
    bright_white: Option<String>,
}

impl Cli {
    pub fn create_schemes_from_cli() -> Result<Vec<HighlightScheme>, regex::Error> {
        let cli = Cli::parse();
        let mut schemes = vec![];

        Self::push_scheme_if_spec_exists(&mut schemes, &cli.black, Color::Black)?;
        Self::push_scheme_if_spec_exists(&mut schemes, &cli.bright_black, Color::BrightBlack)?;

        Self::push_scheme_if_spec_exists(&mut schemes, &cli.red, Color::Red)?;
        Self::push_scheme_if_spec_exists(&mut schemes, &cli.bright_red, Color::BrightRed)?;

        Self::push_scheme_if_spec_exists(&mut schemes, &cli.green, Color::Green)?;
        Self::push_scheme_if_spec_exists(&mut schemes, &cli.bright_green, Color::BrightGreen)?;

        Self::push_scheme_if_spec_exists(&mut schemes, &cli.yellow, Color::Green)?;
        Self::push_scheme_if_spec_exists(&mut schemes, &cli.bright_yellow, Color::BrightYellow)?;

        Self::push_scheme_if_spec_exists(&mut schemes, &cli.blue, Color::Blue)?;
        Self::push_scheme_if_spec_exists(&mut schemes, &cli.bright_blue, Color::BrightBlue)?;

        Self::push_scheme_if_spec_exists(&mut schemes, &cli.magenta, Color::Magenta)?;
        Self::push_scheme_if_spec_exists(&mut schemes, &cli.bright_magenta, Color::BrightMagenta)?;

        Self::push_scheme_if_spec_exists(&mut schemes, &cli.cyan, Color::Cyan)?;
        Self::push_scheme_if_spec_exists(&mut schemes, &cli.bright_cyan, Color::BrightCyan)?;

        Self::push_scheme_if_spec_exists(&mut schemes, &cli.white, Color::White)?;
        Self::push_scheme_if_spec_exists(&mut schemes, &cli.bright_white, Color::BrightWhite)?;
        return Ok(schemes);
    }

    fn push_scheme_if_spec_exists(schemes: &mut Vec<HighlightScheme>, re_str: &Option<String>, color: Color) -> Result<(), regex::Error> {
        if let Some(re_str) = re_str {
            let scheme = HighlightScheme::builder(&re_str)?
                .with_color(color)
                .build();
            schemes.push(scheme);
        }
        return Ok(());
    }
}
