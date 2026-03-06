use std::process;

use clap::Parser;
use exspec_core::extractor::LanguageExtractor;
use exspec_core::output::{compute_exit_code, format_json, format_terminal};
use exspec_core::rules::{evaluate_rules, Config};
use exspec_lang_python::PythonExtractor;
use ignore::WalkBuilder;

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

fn is_python_test_file(path: &str) -> bool {
    let filename = std::path::Path::new(path)
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("");
    (filename.starts_with("test_") || filename.ends_with("_test.py")) && filename.ends_with(".py")
}

fn discover_test_files(root: &str) -> Vec<String> {
    let mut files = Vec::new();
    let walker = WalkBuilder::new(root).hidden(true).git_ignore(true).build();

    for entry in walker.flatten() {
        if entry.file_type().is_some_and(|ft| ft.is_file()) {
            let path = entry.path().to_string_lossy().to_string();
            if is_python_test_file(&path) {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

fn main() {
    let cli = Cli::parse();
    let config = Config::default();
    let extractor = PythonExtractor::new();

    let test_files = discover_test_files(&cli.path);
    let mut all_functions = Vec::new();

    for file_path in &test_files {
        let source = match std::fs::read_to_string(file_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("warning: cannot read {file_path}: {e}");
                continue;
            }
        };
        let funcs = extractor.extract_test_functions(&source, file_path);
        all_functions.extend(funcs);
    }

    let diagnostics = evaluate_rules(&all_functions, &config);

    let output = match cli.format.as_str() {
        "json" => format_json(&diagnostics),
        _ => format_terminal(&diagnostics),
    };

    if !output.is_empty() {
        println!("{output}");
    }

    let exit_code = compute_exit_code(&diagnostics, cli.strict);
    process::exit(exit_code);
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
        assert!(result.is_err());
    }

    // --- File discovery ---

    #[test]
    fn is_python_test_file_matches_test_prefix() {
        assert!(is_python_test_file("tests/test_foo.py"));
        assert!(is_python_test_file("test_bar.py"));
    }

    #[test]
    fn is_python_test_file_matches_test_suffix() {
        assert!(is_python_test_file("foo_test.py"));
    }

    #[test]
    fn is_python_test_file_rejects_non_test() {
        assert!(!is_python_test_file("foo.py"));
        assert!(!is_python_test_file("helper.py"));
        assert!(!is_python_test_file("test_foo.js"));
    }

    #[test]
    fn discover_test_files_finds_test_pattern() {
        let dir = std::env::temp_dir().join(format!("exspec_test_discover_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("test_foo.py"), "").unwrap();
        std::fs::write(dir.join("bar_test.py"), "").unwrap();
        std::fs::write(dir.join("helper.py"), "").unwrap();
        let files = discover_test_files(dir.to_str().unwrap());
        assert_eq!(files.len(), 2);
        assert!(files.iter().all(|f| is_python_test_file(f)));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn discover_test_files_ignores_venv() {
        let files = discover_test_files(".");
        assert!(files.iter().all(|f| !f.contains(".venv")));
    }

    // --- E2E ---

    fn fixture_path(name: &str) -> String {
        format!(
            "{}/tests/fixtures/python/{}",
            env!("CARGO_MANIFEST_DIR").replace("/crates/cli", ""),
            name,
        )
    }

    fn analyze_fixtures(files: &[&str]) -> Vec<exspec_core::rules::Diagnostic> {
        let extractor = PythonExtractor::new();
        let config = Config::default();
        let mut all_functions = Vec::new();
        for name in files {
            let path = fixture_path(name);
            let source = std::fs::read_to_string(&path).unwrap();
            all_functions.extend(extractor.extract_test_functions(&source, &path));
        }
        evaluate_rules(&all_functions, &config)
    }

    #[test]
    fn e2e_t001_violation_detected() {
        let diags = analyze_fixtures(&["t001_violation.py"]);
        assert!(diags.iter().any(|d| d.rule.0 == "T001"));
    }

    #[test]
    fn e2e_t002_violation_detected() {
        let diags = analyze_fixtures(&["t002_violation.py"]);
        assert!(diags.iter().any(|d| d.rule.0 == "T002"));
    }

    #[test]
    fn e2e_t003_violation_detected() {
        let diags = analyze_fixtures(&["t003_violation.py"]);
        assert!(diags.iter().any(|d| d.rule.0 == "T003"));
    }

    #[test]
    fn e2e_pass_files_no_diagnostics() {
        let diags = analyze_fixtures(&["t001_pass.py", "t002_pass.py", "t003_pass.py"]);
        assert!(diags.is_empty(), "expected no diagnostics, got: {diags:?}");
    }
}
