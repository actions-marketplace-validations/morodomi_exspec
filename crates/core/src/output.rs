use crate::rules::{Diagnostic, Severity};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Terminal,
    Json,
    Sarif,
    AiPrompt,
}

pub fn format_terminal(diagnostics: &[Diagnostic]) -> String {
    let mut lines = Vec::new();
    for d in diagnostics {
        let line_str = d.line.map(|l| format!(":{l}")).unwrap_or_default();
        lines.push(format!(
            "{} {}{} {} {}",
            d.severity, d.file, line_str, d.rule, d.message,
        ));
    }
    lines.join("\n")
}

pub fn format_json(diagnostics: &[Diagnostic]) -> String {
    let output = serde_json::json!({
        "diagnostics": diagnostics,
        "metrics": {},
    });
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
    fn terminal_format_block() {
        let output = format_terminal(&[block_diag()]);
        assert_eq!(
            output,
            "BLOCK test.py:10 T001 assertion-free: test has no assertions"
        );
    }

    #[test]
    fn terminal_format_warn() {
        let output = format_terminal(&[warn_diag()]);
        assert_eq!(
            output,
            "WARN test.py:5 T003 giant-test: 73 lines, threshold: 50"
        );
    }

    #[test]
    fn terminal_format_multiple() {
        let output = format_terminal(&[block_diag(), warn_diag()]);
        assert!(output.contains("BLOCK"));
        assert!(output.contains("WARN"));
        assert_eq!(output.lines().count(), 2);
    }

    #[test]
    fn terminal_format_empty() {
        let output = format_terminal(&[]);
        assert_eq!(output, "");
    }

    // --- JSON format ---

    #[test]
    fn json_format_has_diagnostics_and_metrics() {
        let output = format_json(&[block_diag()]);
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert!(parsed["diagnostics"].is_array());
        assert!(parsed["metrics"].is_object());
        assert_eq!(parsed["diagnostics"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn json_format_empty() {
        let output = format_json(&[]);
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert_eq!(parsed["diagnostics"].as_array().unwrap().len(), 0);
    }

    // --- Exit code ---

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
