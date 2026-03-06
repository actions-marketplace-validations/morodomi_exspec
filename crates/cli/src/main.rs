use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "exspec", version, about = "Executable Specification Analyzer")]
pub struct Cli {
    /// Path to analyze
    #[arg(default_value = ".")]
    pub path: String,

    /// Output format
    #[arg(long, default_value = "terminal")]
    pub format: String,

    /// Language filter
    #[arg(long)]
    pub lang: Option<String>,

    /// Treat WARN as errors (exit 1)
    #[arg(long)]
    pub strict: bool,
}

fn main() {
    let _cli = Cli::parse();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_parses_path_argument() {
        let cli = Cli::try_parse_from(["exspec", "."]).unwrap();
        assert_eq!(cli.path, ".");
    }

    #[test]
    fn cli_default_path() {
        let cli = Cli::try_parse_from(["exspec"]).unwrap();
        assert_eq!(cli.path, ".");
    }

    #[test]
    fn cli_strict_flag() {
        let cli = Cli::try_parse_from(["exspec", "--strict", "src/"]).unwrap();
        assert!(cli.strict);
        assert_eq!(cli.path, "src/");
    }

    #[test]
    fn cli_format_option() {
        let cli = Cli::try_parse_from(["exspec", "--format", "json", "."]).unwrap();
        assert_eq!(cli.format, "json");
    }

    #[test]
    fn cli_help_does_not_panic() {
        let result = Cli::try_parse_from(["exspec", "--help"]);
        assert!(result.is_err()); // clap returns Err for --help
    }
}
