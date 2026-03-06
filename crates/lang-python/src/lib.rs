use exspec_core::extractor::{LanguageExtractor, TestFunction};
use exspec_core::rules::Diagnostic;

pub struct PythonExtractor;

impl PythonExtractor {
    pub fn new() -> Self {
        Self
    }

    pub fn parser() -> tree_sitter::Parser {
        let mut parser = tree_sitter::Parser::new();
        let language = tree_sitter_python::LANGUAGE;
        parser
            .set_language(&language.into())
            .expect("failed to load Python grammar");
        parser
    }
}

impl Default for PythonExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageExtractor for PythonExtractor {
    fn extract_test_functions(&self, _source: &str, _file_path: &str) -> Vec<TestFunction> {
        Vec::new()
    }

    fn check(&self, _source: &str, _file_path: &str) -> Vec<Diagnostic> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_python_source() {
        let source = "def test_example():\n    pass\n";
        let mut parser = PythonExtractor::parser();
        let tree = parser.parse(source, None).unwrap();
        assert_eq!(tree.root_node().kind(), "module");
    }

    #[test]
    fn python_extractor_implements_language_extractor() {
        let extractor = PythonExtractor::new();
        let _: &dyn exspec_core::extractor::LanguageExtractor = &extractor;
    }
}
