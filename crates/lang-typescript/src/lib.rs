use exspec_core::extractor::{LanguageExtractor, TestFunction};
use exspec_core::rules::Diagnostic;

pub struct TypeScriptExtractor;

impl TypeScriptExtractor {
    pub fn new() -> Self {
        Self
    }

    pub fn parser() -> tree_sitter::Parser {
        let mut parser = tree_sitter::Parser::new();
        let language = tree_sitter_typescript::LANGUAGE_TYPESCRIPT;
        parser
            .set_language(&language.into())
            .expect("failed to load TypeScript grammar");
        parser
    }
}

impl Default for TypeScriptExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageExtractor for TypeScriptExtractor {
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
    fn parse_typescript_source() {
        let source = "const x: number = 42;\n";
        let mut parser = TypeScriptExtractor::parser();
        let tree = parser.parse(source, None).unwrap();
        assert_eq!(tree.root_node().kind(), "program");
    }

    #[test]
    fn typescript_extractor_implements_language_extractor() {
        let extractor = TypeScriptExtractor::new();
        let _: &dyn exspec_core::extractor::LanguageExtractor = &extractor;
    }
}
