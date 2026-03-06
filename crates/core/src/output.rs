use std::collections::HashSet;

use crate::rules::{Diagnostic, Severity};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Terminal,
    Json,
    Sarif,
    AiPrompt,
}

/// Count unique violated functions by (file, line) pairs.
/// Only per-function diagnostics (line=Some) are counted.
fn count_violated_functions(diagnostics: &[Diagnostic]) -> usize {
    diagnostics
        .iter()
        .filter_map(|d| d.line.map(|l| (d.file.as_str(), l)))
        .collect::<HashSet<_>>()
        .len()
}

pub fn format_terminal(
    diagnostics: &[Diagnostic],
    file_count: usize,
    function_count: usize,
) -> String {
    let mut lines = Vec::new();

    lines.push(format!(
        "exspec v{} -- {} test files, {} test functions",
        env!("CARGO_PKG_VERSION"),
        file_count,
        function_count,
    ));

    if file_count == 0 {
        lines.push("No test files found. Check --lang filter or run from a directory containing test files.".to_string());
    }

    for d in diagnostics {
        let line_str = d.line.map(|l| format!(":{l}")).unwrap_or_default();
        lines.push(format!(
            "{} {}{} {} {}",
            d.severity, d.file, line_str, d.rule, d.message,
        ));
    }

    let block_count = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Block)
        .count();
    let warn_count = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Warn)
        .count();
    let info_count = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Info)
        .count();
    let violated = count_violated_functions(diagnostics);
    let pass_count = function_count.saturating_sub(violated);
    lines.push(format!(
        "Score: BLOCK {block_count} | WARN {warn_count} | INFO {info_count} | PASS {pass_count}",
    ));

    lines.join("\n")
}

pub fn format_json(diagnostics: &[Diagnostic], file_count: usize, function_count: usize) -> String {
    let block_count = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Block)
        .count();
    let warn_count = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Warn)
        .count();
    let info_count = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Info)
        .count();
    let violated = count_violated_functions(diagnostics);
    let pass_count = function_count.saturating_sub(violated);

    let mut output = serde_json::json!({
        "version": env!("CARGO_PKG_VERSION"),
        "summary": {
            "files": file_count,
            "functions": function_count,
            "block": block_count,
            "warn": warn_count,
            "info": info_count,
            "pass": pass_count,
        },
        "diagnostics": diagnostics,
        "metrics": {},
    });

    if file_count == 0 {
        output["guidance"] = serde_json::json!("No test files found. Check --lang filter or run from a directory containing test files.");
    }
    serde_json::to_string_pretty(&output).unwrap_or_else(|_| "{}".to_string())
}

pub fn compute_exit_code(diagnostics: &[Diagnostic], strict: bool) -> i32 {
    for d in diagnostics {
        if d.severity == Severity::Block {
            return 1;
        }
    }
    if strict {
        for d in diagnostics {
            if d.severity == Severity::Warn {
                return 1;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::RuleId;

    fn block_diag() -> Diagnostic {
        Diagnostic {
            rule: RuleId::new("T001"),
            severity: Severity::Block,
            file: "test.py".to_string(),
            line: Some(10),
            message: "assertion-free: test has no assertions".to_string(),
            details: None,
        }
    }

    fn warn_diag() -> Diagnostic {
        Diagnostic {
            rule: RuleId::new("T003"),
            severity: Severity::Warn,
            file: "test.py".to_string(),
            line: Some(5),
            message: "giant-test: 73 lines, threshold: 50".to_string(),
            details: None,
        }
    }

    // --- Terminal format ---

    #[test]
    fn terminal_format_has_summary_header() {
        let output = format_terminal(&[block_diag()], 1, 1);
        assert!(output.starts_with("exspec v"));
        assert!(output.contains("1 test files"));
        assert!(output.contains("1 test functions"));
    }

    #[test]
    fn terminal_format_has_score_footer() {
        let output = format_terminal(&[block_diag()], 1, 1);
        assert!(output.contains("Score: BLOCK 1 | WARN 0 | INFO 0 | PASS 0"));
    }

    #[test]
    fn terminal_format_block() {
        let output = format_terminal(&[block_diag()], 1, 1);
        assert!(output.contains("BLOCK test.py:10 T001 assertion-free: test has no assertions"));
    }

    #[test]
    fn terminal_format_warn() {
        let output = format_terminal(&[warn_diag()], 1, 1);
        assert!(output.contains("WARN test.py:5 T003 giant-test: 73 lines, threshold: 50"));
    }

    #[test]
    fn terminal_format_multiple() {
        let output = format_terminal(&[block_diag(), warn_diag()], 2, 2);
        assert!(output.contains("BLOCK"));
        assert!(output.contains("WARN"));
    }

    #[test]
    fn terminal_format_empty_has_header_and_footer() {
        let output = format_terminal(&[], 0, 0);
        assert!(output.contains("exspec v"));
        assert!(output.contains("Score:"));
    }

    // --- JSON format ---

    #[test]
    fn json_format_has_version_and_summary() {
        let output = format_json(&[block_diag()], 1, 1);
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert!(parsed["version"].is_string());
        assert!(parsed["summary"].is_object());
        assert_eq!(parsed["summary"]["files"], 1);
        assert_eq!(parsed["summary"]["functions"], 1);
        assert_eq!(parsed["summary"]["block"], 1);
        assert_eq!(parsed["summary"]["warn"], 0);
        assert_eq!(parsed["summary"]["pass"], 0);
    }

    #[test]
    fn json_format_has_diagnostics_and_metrics() {
        let output = format_json(&[block_diag()], 1, 1);
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert!(parsed["diagnostics"].is_array());
        assert!(parsed["metrics"].is_object());
        assert_eq!(parsed["diagnostics"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn json_format_empty() {
        let output = format_json(&[], 0, 0);
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert_eq!(parsed["diagnostics"].as_array().unwrap().len(), 0);
        assert_eq!(parsed["summary"]["functions"], 0);
    }

    // --- Exit code ---

    // --- Empty result UX ---

    #[test]
    fn terminal_format_zero_files_shows_guidance() {
        let output = format_terminal(&[], 0, 0);
        assert!(
            output.contains("No test files found"),
            "expected guidance message, got: {output}"
        );
    }

    #[test]
    fn json_format_zero_files_has_guidance() {
        let output = format_json(&[], 0, 0);
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert!(parsed["guidance"].is_string());
    }

    // --- pass_count multi-violation ---

    #[test]
    fn pass_count_with_multi_violation_function() {
        // One function with T001 (BLOCK) + T003 (WARN) should count as 1 violated function, not 2
        let d1 = Diagnostic {
            rule: RuleId::new("T001"),
            severity: Severity::Block,
            file: "test.py".to_string(),
            line: Some(10),
            message: "assertion-free".to_string(),
            details: None,
        };
        let d2 = Diagnostic {
            rule: RuleId::new("T003"),
            severity: Severity::Warn,
            file: "test.py".to_string(),
            line: Some(10), // same function (same file+line)
            message: "giant-test".to_string(),
            details: None,
        };
        // 2 functions total, 1 has 2 violations → pass_count should be 1
        let output = format_terminal(&[d1, d2], 1, 2);
        assert!(output.contains("PASS 1"), "expected PASS 1, got: {output}");
    }

    #[test]
    fn pass_count_excludes_file_level_diagnostics() {
        // File-level diagnostics (line=None) should not count toward violated functions
        let d1 = Diagnostic {
            rule: RuleId::new("T004"),
            severity: Severity::Info,
            file: "test.py".to_string(),
            line: None,
            message: "no-parameterized".to_string(),
            details: None,
        };
        // 1 function total, only file-level diag → pass_count should be 1
        let output = format_terminal(&[d1], 1, 1);
        assert!(output.contains("PASS 1"), "expected PASS 1, got: {output}");
    }

    #[test]
    fn terminal_format_nonzero_files_no_guidance() {
        let output = format_terminal(&[], 1, 0);
        assert!(!output.contains("No test files found"));
    }

    #[test]
    fn exit_code_block_returns_1() {
        assert_eq!(compute_exit_code(&[block_diag()], false), 1);
    }

    #[test]
    fn exit_code_warn_only_returns_0() {
        assert_eq!(compute_exit_code(&[warn_diag()], false), 0);
    }

    #[test]
    fn exit_code_strict_warn_returns_1() {
        assert_eq!(compute_exit_code(&[warn_diag()], true), 1);
    }

    #[test]
    fn exit_code_empty_returns_0() {
        assert_eq!(compute_exit_code(&[], false), 0);
    }
}
