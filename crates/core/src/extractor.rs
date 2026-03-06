use crate::rules::RuleId;

#[derive(Debug, Clone, Default)]
pub struct TestAnalysis {
    pub assertion_count: usize,
    pub mock_count: usize,
    pub mock_classes: Vec<String>,
    pub line_count: usize,
    pub suppressed_rules: Vec<RuleId>,
}

#[derive(Debug, Clone)]
pub struct TestFunction {
    pub name: String,
    pub file: String,
    pub line: usize,
    pub end_line: usize,
    pub analysis: TestAnalysis,
}

pub trait LanguageExtractor {
    fn extract_test_functions(&self, source: &str, file_path: &str) -> Vec<TestFunction>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analysis_default_all_zero_or_empty() {
        let analysis = TestAnalysis::default();
        assert_eq!(analysis.assertion_count, 0);
        assert_eq!(analysis.mock_count, 0);
        assert!(analysis.mock_classes.is_empty());
        assert_eq!(analysis.line_count, 0);
        assert!(analysis.suppressed_rules.is_empty());
    }
}
