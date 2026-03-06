use crate::rules::Diagnostic;

#[derive(Debug, Clone)]
pub struct TestFunction {
    pub name: String,
    pub file: String,
    pub line: usize,
    pub end_line: usize,
}

pub trait LanguageExtractor {
    fn extract_test_functions(&self, source: &str, file_path: &str) -> Vec<TestFunction>;
    fn check(&self, source: &str, file_path: &str) -> Vec<Diagnostic>;
}
