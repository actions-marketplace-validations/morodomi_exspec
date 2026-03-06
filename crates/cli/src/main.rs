use std::process;

use clap::Parser;
use exspec_core::extractor::LanguageExtractor;
use exspec_core::output::{compute_exit_code, format_json, format_terminal};
use exspec_core::rules::{evaluate_rules, Config};
use exspec_lang_python::PythonExtractor;
use exspec_lang_typescript::TypeScriptExtractor;
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

    /// Language filter (python, typescript)
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

fn is_typescript_test_file(path: &str) -> bool {
    let filename = std::path::Path::new(path)
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("");
    filename.ends_with(".test.ts")
        || filename.ends_with(".test.tsx")
        || filename.ends_with(".spec.ts")
        || filename.ends_with(".spec.tsx")
}

fn discover_test_files(root: &str, lang: Option<&str>) -> (Vec<String>, Vec<String>) {
    let mut python_files = Vec::new();
    let mut ts_files = Vec::new();
    let walker = WalkBuilder::new(root).hidden(true).git_ignore(true).build();

    let include_python = lang.is_none() || lang == Some("python");
    let include_ts = lang.is_none() || lang == Some("typescript");

    for entry in walker.flatten() {
        if entry.file_type().is_some_and(|ft| ft.is_file()) {
            let path = entry.path().to_string_lossy().to_string();
            if include_python && is_python_test_file(&path) {
                python_files.push(path);
            } else if include_ts && is_typescript_test_file(&path) {
                ts_files.push(path);
            }
        }
    }
    python_files.sort();
    ts_files.sort();
    (python_files, ts_files)
}

fn main() {
    let cli = Cli::parse();
    let config = Config::default();
    let py_extractor = PythonExtractor::new();
    let ts_extractor = TypeScriptExtractor::new();

    let (python_files, ts_files) = discover_test_files(&cli.path, cli.lang.as_deref());
    let total_files = python_files.len() + ts_files.len();
    let mut all_functions = Vec::new();

    for file_path in &python_files {
        let source = match std::fs::read_to_string(file_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("warning: cannot read {file_path}: {e}");
                continue;
            }
        };
        let funcs = py_extractor.extract_test_functions(&source, file_path);
        all_functions.extend(funcs);
    }

    for file_path in &ts_files {
        let source = match std::fs::read_to_string(file_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("warning: cannot read {file_path}: {e}");
                continue;
            }
        };
        let funcs = ts_extractor.extract_test_functions(&source, file_path);
        all_functions.extend(funcs);
    }

    let diagnostics = evaluate_rules(&all_functions, &config);

    let output = match cli.format.as_str() {
        "json" => format_json(&diagnostics, total_files, all_functions.len()),
        _ => format_terminal(&diagnostics, total_files, all_functions.len()),
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
    fn cli_lang_option() {
        let cli = Cli::try_parse_from(["exspec", "--lang", "python", "."]).unwrap();
        assert_eq!(cli.lang, Some("python".to_string()));
    }

    #[test]
    fn cli_help_does_not_panic() {
        let result = Cli::try_parse_from(["exspec", "--help"]);
        assert!(result.is_err());
    }

    // --- Python file discovery ---

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

    // --- TypeScript file discovery ---

    #[test]
    fn is_typescript_test_file_matches_test_patterns() {
        assert!(is_typescript_test_file("foo.test.ts"));
        assert!(is_typescript_test_file("bar.spec.ts"));
        assert!(is_typescript_test_file("baz.test.tsx"));
        assert!(is_typescript_test_file("qux.spec.tsx"));
    }

    #[test]
    fn is_typescript_test_file_rejects_non_test() {
        assert!(!is_typescript_test_file("foo.ts"));
        assert!(!is_typescript_test_file("helper.ts"));
        assert!(!is_typescript_test_file("test.js"));
    }

    // --- Multi-language discovery ---

    #[test]
    fn discover_test_files_finds_test_pattern() {
        let dir = std::env::temp_dir().join(format!("exspec_test_discover_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("test_foo.py"), "").unwrap();
        std::fs::write(dir.join("bar_test.py"), "").unwrap();
        std::fs::write(dir.join("helper.py"), "").unwrap();
        std::fs::write(dir.join("baz.test.ts"), "").unwrap();
        let (py, ts) = discover_test_files(dir.to_str().unwrap(), None);
        assert_eq!(py.len(), 2);
        assert_eq!(ts.len(), 1);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn discover_test_files_lang_filter_python() {
        let dir = std::env::temp_dir().join(format!("exspec_test_lang_py_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("test_foo.py"), "").unwrap();
        std::fs::write(dir.join("baz.test.ts"), "").unwrap();
        let (py, ts) = discover_test_files(dir.to_str().unwrap(), Some("python"));
        assert_eq!(py.len(), 1);
        assert_eq!(ts.len(), 0);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn discover_test_files_lang_filter_typescript() {
        let dir = std::env::temp_dir().join(format!("exspec_test_lang_ts_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("test_foo.py"), "").unwrap();
        std::fs::write(dir.join("baz.test.ts"), "").unwrap();
        let (py, ts) = discover_test_files(dir.to_str().unwrap(), Some("typescript"));
        assert_eq!(py.len(), 0);
        assert_eq!(ts.len(), 1);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn discover_test_files_ignores_venv() {
        let (py, _) = discover_test_files(".", None);
        assert!(py.iter().all(|f| !f.contains(".venv")));
    }

    // --- E2E ---

    fn fixture_path(lang: &str, name: &str) -> String {
        format!(
            "{}/tests/fixtures/{}/{}",
            env!("CARGO_MANIFEST_DIR").replace("/crates/cli", ""),
            lang,
            name,
        )
    }

    fn analyze_python_fixtures(files: &[&str]) -> Vec<exspec_core::rules::Diagnostic> {
        let extractor = PythonExtractor::new();
        let config = Config::default();
        let mut all_functions = Vec::new();
        for name in files {
            let path = fixture_path("python", name);
            let source = std::fs::read_to_string(&path).unwrap();
            all_functions.extend(extractor.extract_test_functions(&source, &path));
        }
        evaluate_rules(&all_functions, &config)
    }

    fn analyze_ts_fixtures(files: &[&str]) -> Vec<exspec_core::rules::Diagnostic> {
        let extractor = TypeScriptExtractor::new();
        let config = Config::default();
        let mut all_functions = Vec::new();
        for name in files {
            let path = fixture_path("typescript", name);
            let source = std::fs::read_to_string(&path).unwrap();
            all_functions.extend(extractor.extract_test_functions(&source, &path));
        }
        evaluate_rules(&all_functions, &config)
    }

    // Python E2E
    #[test]
    fn e2e_t001_violation_detected() {
        let diags = analyze_python_fixtures(&["t001_violation.py"]);
        assert!(diags.iter().any(|d| d.rule.0 == "T001"));
    }

    #[test]
    fn e2e_t002_violation_detected() {
        let diags = analyze_python_fixtures(&["t002_violation.py"]);
        assert!(diags.iter().any(|d| d.rule.0 == "T002"));
    }

    #[test]
    fn e2e_t003_violation_detected() {
        let diags = analyze_python_fixtures(&["t003_violation.py"]);
        assert!(diags.iter().any(|d| d.rule.0 == "T003"));
    }

    #[test]
    fn e2e_pass_files_no_diagnostics() {
        let diags = analyze_python_fixtures(&["t001_pass.py", "t002_pass.py", "t003_pass.py"]);
        assert!(diags.is_empty(), "expected no diagnostics, got: {diags:?}");
    }

    // TypeScript E2E
    #[test]
    fn e2e_ts_t001_violation_detected() {
        let diags = analyze_ts_fixtures(&["t001_violation.test.ts"]);
        assert!(diags.iter().any(|d| d.rule.0 == "T001"));
    }

    #[test]
    fn e2e_ts_t002_violation_detected() {
        let diags = analyze_ts_fixtures(&["t002_violation.test.ts"]);
        assert!(diags.iter().any(|d| d.rule.0 == "T002"));
    }

    #[test]
    fn e2e_ts_t003_violation_detected() {
        let diags = analyze_ts_fixtures(&["t003_violation.test.ts"]);
        assert!(diags.iter().any(|d| d.rule.0 == "T003"));
    }

    #[test]
    fn e2e_ts_pass_files_no_diagnostics() {
        let diags = analyze_ts_fixtures(&[
            "t001_pass.test.ts",
            "t002_pass.test.ts",
            "t003_pass.test.ts",
        ]);
        assert!(diags.is_empty(), "expected no diagnostics, got: {diags:?}");
    }

    // Suppression E2E
    #[test]
    fn e2e_python_suppression_hides_t002() {
        let diags = analyze_python_fixtures(&["suppressed.py"]);
        assert!(
            !diags.iter().any(|d| d.rule.0 == "T002"),
            "T002 should be suppressed"
        );
    }

    #[test]
    fn e2e_ts_suppression_hides_t002() {
        let diags = analyze_ts_fixtures(&["suppressed.test.ts"]);
        assert!(
            !diags.iter().any(|d| d.rule.0 == "T002"),
            "T002 should be suppressed"
        );
    }
}
