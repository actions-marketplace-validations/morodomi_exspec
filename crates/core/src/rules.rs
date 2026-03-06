use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Warn,
    Block,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Block => "BLOCK",
            Severity::Warn => "WARN",
            Severity::Info => "INFO",
        }
    }

    pub fn exit_code(&self) -> i32 {
        match self {
            Severity::Block => 1,
            Severity::Warn => 0,
            Severity::Info => 0,
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Severity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BLOCK" => Ok(Severity::Block),
            "WARN" => Ok(Severity::Warn),
            "INFO" => Ok(Severity::Info),
            _ => Err(format!("unknown severity: {s}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RuleId(pub String);

impl RuleId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl fmt::Display for RuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub rule: RuleId,
    pub severity: Severity,
    pub file: String,
    pub line: Option<usize>,
    pub message: String,
    pub details: Option<String>,
}

pub struct Config {
    pub mock_max: usize,
    pub mock_class_max: usize,
    pub test_max_lines: usize,
    pub disabled_rules: Vec<RuleId>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mock_max: 5,
            mock_class_max: 3,
            test_max_lines: 50,
            disabled_rules: Vec::new(),
        }
    }
}

use crate::extractor::TestFunction;

pub fn evaluate_rules(functions: &[TestFunction], config: &Config) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    for func in functions {
        let analysis = &func.analysis;

        // T001: assertion-free
        if !is_disabled(config, "T001")
            && !is_suppressed(analysis, "T001")
            && analysis.assertion_count == 0
        {
            diagnostics.push(Diagnostic {
                rule: RuleId::new("T001"),
                severity: Severity::Block,
                file: func.file.clone(),
                line: Some(func.line),
                message: "assertion-free: test has no assertions".to_string(),
                details: None,
            });
        }

        // T002: mock-overuse
        if !is_disabled(config, "T002")
            && !is_suppressed(analysis, "T002")
            && (analysis.mock_count > config.mock_max
                || analysis.mock_classes.len() > config.mock_class_max)
        {
            diagnostics.push(Diagnostic {
                rule: RuleId::new("T002"),
                severity: Severity::Warn,
                file: func.file.clone(),
                line: Some(func.line),
                message: format!(
                    "mock-overuse: {} mocks ({} classes), threshold: {} mocks / {} classes",
                    analysis.mock_count,
                    analysis.mock_classes.len(),
                    config.mock_max,
                    config.mock_class_max,
                ),
                details: None,
            });
        }

        // T003: giant-test
        if !is_disabled(config, "T003")
            && !is_suppressed(analysis, "T003")
            && analysis.line_count > config.test_max_lines
        {
            diagnostics.push(Diagnostic {
                rule: RuleId::new("T003"),
                severity: Severity::Warn,
                file: func.file.clone(),
                line: Some(func.line),
                message: format!(
                    "giant-test: {} lines, threshold: {}",
                    analysis.line_count, config.test_max_lines,
                ),
                details: None,
            });
        }
    }

    diagnostics
}

fn is_disabled(config: &Config, rule_id: &str) -> bool {
    config.disabled_rules.iter().any(|r| r.0 == rule_id)
}

fn is_suppressed(analysis: &crate::extractor::TestAnalysis, rule_id: &str) -> bool {
    analysis.suppressed_rules.iter().any(|r| r.0 == rule_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extractor::{TestAnalysis, TestFunction};

    fn make_func(name: &str, analysis: TestAnalysis) -> TestFunction {
        TestFunction {
            name: name.to_string(),
            file: "test.py".to_string(),
            line: 1,
            end_line: 10,
            analysis,
        }
    }

    // --- Severity tests (from Phase 1) ---

    #[test]
    fn severity_ordering() {
        assert!(Severity::Block > Severity::Warn);
        assert!(Severity::Warn > Severity::Info);
    }

    #[test]
    fn severity_as_str_roundtrip() {
        for severity in [Severity::Block, Severity::Warn, Severity::Info] {
            let s = severity.as_str();
            let parsed = Severity::from_str(s).unwrap();
            assert_eq!(parsed, severity);
        }
    }

    #[test]
    fn severity_to_exit_code() {
        assert_eq!(Severity::Block.exit_code(), 1);
        assert_eq!(Severity::Warn.exit_code(), 0);
        assert_eq!(Severity::Info.exit_code(), 0);
    }

    #[test]
    fn severity_from_str_invalid() {
        assert!(Severity::from_str("UNKNOWN").is_err());
    }

    #[test]
    fn rule_id_display() {
        let id = RuleId::new("T001");
        assert_eq!(id.to_string(), "T001");
    }

    // --- T001: assertion-free ---

    #[test]
    fn t001_assertion_count_zero_produces_block() {
        let funcs = vec![make_func(
            "test_no_assert",
            TestAnalysis {
                assertion_count: 0,
                ..Default::default()
            },
        )];
        let diags = evaluate_rules(&funcs, &Config::default());
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, RuleId::new("T001"));
        assert_eq!(diags[0].severity, Severity::Block);
    }

    #[test]
    fn t001_assertion_count_positive_no_diagnostic() {
        let funcs = vec![make_func(
            "test_with_assert",
            TestAnalysis {
                assertion_count: 1,
                ..Default::default()
            },
        )];
        let diags = evaluate_rules(&funcs, &Config::default());
        assert!(diags.is_empty());
    }

    // --- T002: mock-overuse ---

    #[test]
    fn t002_mock_count_exceeds_threshold_produces_warn() {
        let funcs = vec![make_func(
            "test_many_mocks",
            TestAnalysis {
                assertion_count: 1,
                mock_count: 6,
                mock_classes: vec![
                    "a".into(),
                    "b".into(),
                    "c".into(),
                    "d".into(),
                    "e".into(),
                    "f".into(),
                ],
                ..Default::default()
            },
        )];
        let diags = evaluate_rules(&funcs, &Config::default());
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, RuleId::new("T002"));
        assert_eq!(diags[0].severity, Severity::Warn);
    }

    #[test]
    fn t002_mock_count_within_threshold_no_diagnostic() {
        let funcs = vec![make_func(
            "test_few_mocks",
            TestAnalysis {
                assertion_count: 1,
                mock_count: 2,
                mock_classes: vec!["db".into()],
                ..Default::default()
            },
        )];
        let diags = evaluate_rules(&funcs, &Config::default());
        assert!(diags.is_empty());
    }

    #[test]
    fn t002_mock_class_count_exceeds_threshold_alone_produces_warn() {
        let funcs = vec![make_func(
            "test_many_classes",
            TestAnalysis {
                assertion_count: 1,
                mock_count: 4, // within mock_max=5
                mock_classes: vec!["a".into(), "b".into(), "c".into(), "d".into()], // > mock_class_max=3
                ..Default::default()
            },
        )];
        let diags = evaluate_rules(&funcs, &Config::default());
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, RuleId::new("T002"));
    }

    // --- T003: giant-test ---

    #[test]
    fn t003_line_count_exceeds_threshold_produces_warn() {
        let funcs = vec![make_func(
            "test_giant",
            TestAnalysis {
                assertion_count: 1,
                line_count: 73,
                ..Default::default()
            },
        )];
        let diags = evaluate_rules(&funcs, &Config::default());
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, RuleId::new("T003"));
        assert_eq!(diags[0].severity, Severity::Warn);
    }

    #[test]
    fn t003_line_count_at_threshold_no_diagnostic() {
        let funcs = vec![make_func(
            "test_boundary",
            TestAnalysis {
                assertion_count: 1,
                line_count: 50, // exactly at threshold, strict >
                ..Default::default()
            },
        )];
        let diags = evaluate_rules(&funcs, &Config::default());
        assert!(diags.is_empty());
    }

    // --- Config disabled ---

    #[test]
    fn disabled_rule_not_reported() {
        let funcs = vec![make_func(
            "test_no_assert",
            TestAnalysis {
                assertion_count: 0,
                ..Default::default()
            },
        )];
        let config = Config {
            disabled_rules: vec![RuleId::new("T001")],
            ..Config::default()
        };
        let diags = evaluate_rules(&funcs, &config);
        assert!(diags.is_empty());
    }

    // --- Suppression ---

    #[test]
    fn suppressed_rule_not_reported() {
        let funcs = vec![make_func(
            "test_many_mocks",
            TestAnalysis {
                assertion_count: 1,
                mock_count: 6,
                mock_classes: vec![
                    "a".into(),
                    "b".into(),
                    "c".into(),
                    "d".into(),
                    "e".into(),
                    "f".into(),
                ],
                suppressed_rules: vec![RuleId::new("T002")],
                ..Default::default()
            },
        )];
        let diags = evaluate_rules(&funcs, &Config::default());
        assert!(diags.is_empty());
    }

    // --- Multiple violations ---

    #[test]
    fn multiple_violations_reported() {
        let funcs = vec![make_func(
            "test_bad",
            TestAnalysis {
                assertion_count: 0,
                line_count: 73,
                ..Default::default()
            },
        )];
        let diags = evaluate_rules(&funcs, &Config::default());
        assert_eq!(diags.len(), 2);
        let rule_ids: Vec<&str> = diags.iter().map(|d| d.rule.0.as_str()).collect();
        assert!(rule_ids.contains(&"T001"));
        assert!(rule_ids.contains(&"T003"));
    }
}
