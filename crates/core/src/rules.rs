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

pub trait Rule {
    fn id(&self) -> &RuleId;
    fn severity(&self) -> Severity;
    fn name(&self) -> &str;
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
