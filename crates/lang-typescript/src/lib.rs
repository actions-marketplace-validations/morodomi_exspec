use std::collections::BTreeSet;
use std::sync::OnceLock;

use exspec_core::extractor::{LanguageExtractor, TestAnalysis, TestFunction};
use exspec_core::suppress::parse_suppression;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Node, Parser, Query, QueryCursor};

const TEST_FUNCTION_QUERY: &str = include_str!("../queries/test_function.scm");
const ASSERTION_QUERY: &str = include_str!("../queries/assertion.scm");
const MOCK_USAGE_QUERY: &str = include_str!("../queries/mock_usage.scm");
const MOCK_ASSIGNMENT_QUERY: &str = include_str!("../queries/mock_assignment.scm");

fn ts_language() -> tree_sitter::Language {
    tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()
}

fn cached_query<'a>(lock: &'a OnceLock<Query>, source: &str) -> &'a Query {
    lock.get_or_init(|| Query::new(&ts_language(), source).expect("invalid query"))
}

static TEST_QUERY_CACHE: OnceLock<Query> = OnceLock::new();
static ASSERTION_QUERY_CACHE: OnceLock<Query> = OnceLock::new();
static MOCK_QUERY_CACHE: OnceLock<Query> = OnceLock::new();
static MOCK_ASSIGN_QUERY_CACHE: OnceLock<Query> = OnceLock::new();

pub struct TypeScriptExtractor;

impl TypeScriptExtractor {
    pub fn new() -> Self {
        Self
    }

    pub fn parser() -> Parser {
        let mut parser = Parser::new();
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

fn count_captures(query: &Query, capture_name: &str, node: Node, source: &[u8]) -> usize {
    let idx = query
        .capture_index_for_name(capture_name)
        .expect("capture not found");
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, node, source);
    let mut count = 0;
    while let Some(m) = matches.next() {
        count += m.captures.iter().filter(|c| c.index == idx).count();
    }
    count
}

fn collect_mock_class_names(query: &Query, node: Node, source: &[u8]) -> Vec<String> {
    let var_idx = query
        .capture_index_for_name("var_name")
        .expect("no @var_name capture");
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, node, source);
    let mut names = BTreeSet::new();
    while let Some(m) = matches.next() {
        for c in m.captures.iter().filter(|c| c.index == var_idx) {
            if let Ok(var) = c.node.utf8_text(source) {
                names.insert(extract_mock_class_name(var));
            }
        }
    }
    names.into_iter().collect()
}

fn extract_mock_class_name(var_name: &str) -> String {
    // camelCase: strip "mock" prefix and lowercase first char
    if let Some(stripped) = var_name.strip_prefix("mock") {
        if !stripped.is_empty() && stripped.starts_with(|c: char| c.is_uppercase()) {
            return stripped.to_string();
        }
    }
    var_name.to_string()
}

fn extract_suppression_from_previous_line(
    source: &str,
    start_row: usize,
) -> Vec<exspec_core::rules::RuleId> {
    if start_row == 0 {
        return Vec::new();
    }
    let lines: Vec<&str> = source.lines().collect();
    let prev_line = lines.get(start_row - 1).unwrap_or(&"");
    parse_suppression(prev_line)
}

struct TestMatch {
    name: String,
    fn_start_byte: usize,
    fn_end_byte: usize,
    fn_start_row: usize,
    fn_end_row: usize,
}

impl LanguageExtractor for TypeScriptExtractor {
    fn extract_test_functions(&self, source: &str, file_path: &str) -> Vec<TestFunction> {
        let mut parser = Self::parser();
        let tree = match parser.parse(source, None) {
            Some(t) => t,
            None => return Vec::new(),
        };

        let test_query = cached_query(&TEST_QUERY_CACHE, TEST_FUNCTION_QUERY);
        let assertion_query = cached_query(&ASSERTION_QUERY_CACHE, ASSERTION_QUERY);
        let mock_query = cached_query(&MOCK_QUERY_CACHE, MOCK_USAGE_QUERY);
        let mock_assign_query = cached_query(&MOCK_ASSIGN_QUERY_CACHE, MOCK_ASSIGNMENT_QUERY);

        let name_idx = test_query
            .capture_index_for_name("name")
            .expect("no @name capture");
        let function_idx = test_query
            .capture_index_for_name("function")
            .expect("no @function capture");

        let source_bytes = source.as_bytes();
        let root = tree.root_node();

        let mut test_matches = Vec::new();
        {
            let mut cursor = QueryCursor::new();
            let mut matches = cursor.matches(test_query, root, source_bytes);
            while let Some(m) = matches.next() {
                let name_capture = match m.captures.iter().find(|c| c.index == name_idx) {
                    Some(c) => c,
                    None => continue,
                };
                let name = match name_capture.node.utf8_text(source_bytes) {
                    Ok(s) => s.to_string(),
                    Err(_) => continue,
                };

                let fn_capture = match m.captures.iter().find(|c| c.index == function_idx) {
                    Some(c) => c,
                    None => continue,
                };

                test_matches.push(TestMatch {
                    name,
                    fn_start_byte: fn_capture.node.start_byte(),
                    fn_end_byte: fn_capture.node.end_byte(),
                    fn_start_row: fn_capture.node.start_position().row,
                    fn_end_row: fn_capture.node.end_position().row,
                });
            }
        }

        let mut functions = Vec::new();
        for tm in &test_matches {
            let fn_node = match root.descendant_for_byte_range(tm.fn_start_byte, tm.fn_end_byte) {
                Some(n) => n,
                None => continue,
            };

            let line = tm.fn_start_row + 1;
            let end_line = tm.fn_end_row + 1;
            let line_count = end_line - line + 1;

            let assertion_count =
                count_captures(assertion_query, "assertion", fn_node, source_bytes);
            let mock_count = count_captures(mock_query, "mock", fn_node, source_bytes);
            let mock_classes = collect_mock_class_names(mock_assign_query, fn_node, source_bytes);

            let suppressed_rules = extract_suppression_from_previous_line(source, tm.fn_start_row);

            functions.push(TestFunction {
                name: tm.name.clone(),
                file: file_path.to_string(),
                line,
                end_line,
                analysis: TestAnalysis {
                    assertion_count,
                    mock_count,
                    mock_classes,
                    line_count,
                    suppressed_rules,
                },
            });
        }

        functions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> String {
        let path = format!(
            "{}/tests/fixtures/typescript/{}",
            env!("CARGO_MANIFEST_DIR").replace("/crates/lang-typescript", ""),
            name
        );
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {path}: {e}"))
    }

    // --- Phase 1 preserved tests ---

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

    // --- Cycle 1: Test function extraction ---

    #[test]
    fn extract_single_test_function() {
        let source = fixture("t001_pass.test.ts");
        let extractor = TypeScriptExtractor::new();
        let funcs = extractor.extract_test_functions(&source, "t001_pass.test.ts");
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].name, "create user");
        assert_eq!(funcs[0].line, 1);
    }

    #[test]
    fn extract_multiple_tests_excludes_helpers_and_describe() {
        let source = fixture("multiple_tests.test.ts");
        let extractor = TypeScriptExtractor::new();
        let funcs = extractor.extract_test_functions(&source, "multiple_tests.test.ts");
        assert_eq!(funcs.len(), 3);
        let names: Vec<&str> = funcs.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(
            names,
            vec!["adds numbers", "subtracts numbers", "multiplies numbers"]
        );
    }

    #[test]
    fn line_count_calculation() {
        let source = fixture("t001_pass.test.ts");
        let extractor = TypeScriptExtractor::new();
        let funcs = extractor.extract_test_functions(&source, "t001_pass.test.ts");
        assert_eq!(
            funcs[0].analysis.line_count,
            funcs[0].end_line - funcs[0].line + 1
        );
    }

    #[test]
    fn violation_file_extracts_function() {
        let source = fixture("t001_violation.test.ts");
        let extractor = TypeScriptExtractor::new();
        let funcs = extractor.extract_test_functions(&source, "t001_violation.test.ts");
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].name, "create user");
    }

    // --- Cycle 2: Assertion detection ---

    #[test]
    fn assertion_count_zero_for_violation() {
        let source = fixture("t001_violation.test.ts");
        let extractor = TypeScriptExtractor::new();
        let funcs = extractor.extract_test_functions(&source, "t001_violation.test.ts");
        assert_eq!(funcs[0].analysis.assertion_count, 0);
    }

    #[test]
    fn assertion_count_positive_for_pass() {
        let source = fixture("t001_pass.test.ts");
        let extractor = TypeScriptExtractor::new();
        let funcs = extractor.extract_test_functions(&source, "t001_pass.test.ts");
        assert!(funcs[0].analysis.assertion_count >= 1);
    }

    // --- Cycle 2: Mock detection ---

    #[test]
    fn mock_count_for_violation() {
        let source = fixture("t002_violation.test.ts");
        let extractor = TypeScriptExtractor::new();
        let funcs = extractor.extract_test_functions(&source, "t002_violation.test.ts");
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].analysis.mock_count, 6);
    }

    #[test]
    fn mock_count_for_pass() {
        let source = fixture("t002_pass.test.ts");
        let extractor = TypeScriptExtractor::new();
        let funcs = extractor.extract_test_functions(&source, "t002_pass.test.ts");
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].analysis.mock_count, 1);
        assert_eq!(funcs[0].analysis.mock_classes, vec!["Db"]);
    }

    #[test]
    fn mock_class_name_extraction() {
        assert_eq!(extract_mock_class_name("mockDb"), "Db");
        assert_eq!(
            extract_mock_class_name("mockPaymentService"),
            "PaymentService"
        );
        assert_eq!(extract_mock_class_name("myMock"), "myMock");
    }

    // --- Inline suppression ---

    #[test]
    fn suppressed_test_has_suppressed_rules() {
        let source = fixture("suppressed.test.ts");
        let extractor = TypeScriptExtractor::new();
        let funcs = extractor.extract_test_functions(&source, "suppressed.test.ts");
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].analysis.mock_count, 6);
        assert!(funcs[0]
            .analysis
            .suppressed_rules
            .iter()
            .any(|r| r.0 == "T002"));
    }

    #[test]
    fn non_suppressed_test_has_empty_suppressed_rules() {
        let source = fixture("t002_violation.test.ts");
        let extractor = TypeScriptExtractor::new();
        let funcs = extractor.extract_test_functions(&source, "t002_violation.test.ts");
        assert!(funcs[0].analysis.suppressed_rules.is_empty());
    }

    // --- Giant test ---

    #[test]
    fn giant_test_line_count() {
        let source = fixture("t003_violation.test.ts");
        let extractor = TypeScriptExtractor::new();
        let funcs = extractor.extract_test_functions(&source, "t003_violation.test.ts");
        assert_eq!(funcs.len(), 1);
        assert!(funcs[0].analysis.line_count > 50);
    }

    #[test]
    fn short_test_line_count() {
        let source = fixture("t003_pass.test.ts");
        let extractor = TypeScriptExtractor::new();
        let funcs = extractor.extract_test_functions(&source, "t003_pass.test.ts");
        assert_eq!(funcs.len(), 1);
        assert!(funcs[0].analysis.line_count <= 50);
    }
}
