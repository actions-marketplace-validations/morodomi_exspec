use std::collections::BTreeSet;

use exspec_core::extractor::{LanguageExtractor, TestAnalysis, TestFunction};
use streaming_iterator::StreamingIterator;
use tree_sitter::{Node, Parser, Query, QueryCursor};

const TEST_FUNCTION_QUERY: &str = include_str!("../queries/test_function.scm");
const ASSERTION_QUERY: &str = include_str!("../queries/assertion.scm");
const MOCK_USAGE_QUERY: &str = include_str!("../queries/mock_usage.scm");
const MOCK_ASSIGNMENT_QUERY: &str = include_str!("../queries/mock_assignment.scm");

pub struct PythonExtractor;

impl PythonExtractor {
    pub fn new() -> Self {
        Self
    }

    pub fn parser() -> Parser {
        let mut parser = Parser::new();
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

struct TestMatch {
    name: String,
    fn_node_id: usize,
    fn_start_row: usize,
    fn_end_row: usize,
    decorated_node_id: Option<usize>,
}

impl LanguageExtractor for PythonExtractor {
    fn extract_test_functions(&self, source: &str, file_path: &str) -> Vec<TestFunction> {
        let mut parser = Self::parser();
        let tree = match parser.parse(source, None) {
            Some(t) => t,
            None => return Vec::new(),
        };

        let lang: tree_sitter::Language = tree_sitter_python::LANGUAGE.into();
        let test_query = Query::new(&lang, TEST_FUNCTION_QUERY).expect("invalid test_function.scm");
        let assertion_query = Query::new(&lang, ASSERTION_QUERY).expect("invalid assertion.scm");
        let mock_query = Query::new(&lang, MOCK_USAGE_QUERY).expect("invalid mock_usage.scm");
        let mock_assign_query =
            Query::new(&lang, MOCK_ASSIGNMENT_QUERY).expect("invalid mock_assignment.scm");

        let name_idx = test_query
            .capture_index_for_name("name")
            .expect("no @name capture");
        let function_idx = test_query
            .capture_index_for_name("function")
            .expect("no @function capture");
        let decorated_idx = test_query
            .capture_index_for_name("decorated")
            .expect("no @decorated capture");

        let source_bytes = source.as_bytes();
        let root = tree.root_node();

        // Collect test matches first (StreamingIterator borrows cursor)
        // Decorated matches take priority; track fn_node_ids to deduplicate.
        let mut test_matches = Vec::new();
        let mut decorated_fn_ids = std::collections::HashSet::new();
        {
            let mut cursor = QueryCursor::new();
            let mut matches = cursor.matches(&test_query, root, source_bytes);
            while let Some(m) = matches.next() {
                let name_capture = match m.captures.iter().find(|c| c.index == name_idx) {
                    Some(c) => c,
                    None => continue,
                };
                let name = match name_capture.node.utf8_text(source_bytes) {
                    Ok(s) => s.to_string(),
                    Err(_) => continue,
                };

                let decorated_capture = m.captures.iter().find(|c| c.index == decorated_idx);
                let fn_capture = m.captures.iter().find(|c| c.index == function_idx);

                if let Some(dec) = decorated_capture {
                    let inner_fn = dec
                        .node
                        .child_by_field_name("definition")
                        .unwrap_or(dec.node);
                    decorated_fn_ids.insert(inner_fn.id());
                    test_matches.push(TestMatch {
                        name,
                        fn_node_id: inner_fn.id(),
                        fn_start_row: inner_fn.start_position().row,
                        fn_end_row: inner_fn.end_position().row,
                        decorated_node_id: Some(dec.node.id()),
                    });
                } else if let Some(fn_c) = fn_capture {
                    test_matches.push(TestMatch {
                        name,
                        fn_node_id: fn_c.node.id(),
                        fn_start_row: fn_c.node.start_position().row,
                        fn_end_row: fn_c.node.end_position().row,
                        decorated_node_id: None,
                    });
                }
            }
        }

        // Remove bare function matches that are already covered by decorated matches
        test_matches.retain(|tm| {
            tm.decorated_node_id.is_some() || !decorated_fn_ids.contains(&tm.fn_node_id)
        });

        // Now resolve nodes and build TestFunction entries
        let mut functions = Vec::new();
        for tm in &test_matches {
            let fn_node = find_node_by_id(root, tm.fn_node_id);
            let decorated_node = tm
                .decorated_node_id
                .and_then(|id| find_node_by_id(root, id));

            let fn_node = match fn_node {
                Some(n) => n,
                None => continue,
            };

            let line = tm.fn_start_row + 1;
            let end_line = tm.fn_end_row + 1;
            let line_count = end_line - line + 1;

            let assertion_count =
                count_captures(&assertion_query, "assertion", fn_node, source_bytes);

            let mock_scope = decorated_node.unwrap_or(fn_node);
            let mock_count = count_captures(&mock_query, "mock", mock_scope, source_bytes);

            let mock_classes = collect_mock_class_names(&mock_assign_query, fn_node, source_bytes);

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
                    suppressed_rules: Vec::new(),
                },
            });
        }

        functions
    }
}

fn find_node_by_id(root: Node, target_id: usize) -> Option<Node> {
    if root.id() == target_id {
        return Some(root);
    }
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if let Some(found) = find_node_by_id(child, target_id) {
            return Some(found);
        }
    }
    None
}

fn extract_mock_class_name(var_name: &str) -> String {
    if let Some(stripped) = var_name.strip_prefix("mock_") {
        if !stripped.is_empty() {
            return stripped.to_string();
        }
    }
    var_name.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> String {
        let path = format!(
            "{}/tests/fixtures/python/{}",
            env!("CARGO_MANIFEST_DIR").replace("/crates/lang-python", ""),
            name
        );
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {path}: {e}"))
    }

    // --- Cycle 2: Test function extraction ---

    #[test]
    fn extract_single_test_function() {
        let source = fixture("t001_pass.py");
        let extractor = PythonExtractor::new();
        let funcs = extractor.extract_test_functions(&source, "t001_pass.py");
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].name, "test_create_user");
        assert_eq!(funcs[0].line, 1);
    }

    #[test]
    fn extract_multiple_test_functions_excludes_helpers() {
        let source = fixture("multiple_tests.py");
        let extractor = PythonExtractor::new();
        let funcs = extractor.extract_test_functions(&source, "multiple_tests.py");
        assert_eq!(funcs.len(), 3);
        let names: Vec<&str> = funcs.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(names, vec!["test_first", "test_second", "test_third"]);
        assert!(!names.contains(&"helper"));
    }

    #[test]
    fn line_count_calculation() {
        let source = fixture("t001_pass.py");
        let extractor = PythonExtractor::new();
        let funcs = extractor.extract_test_functions(&source, "t001_pass.py");
        assert_eq!(
            funcs[0].analysis.line_count,
            funcs[0].end_line - funcs[0].line + 1
        );
    }

    // --- Cycle 3: Assertion detection ---

    #[test]
    fn assertion_count_zero_for_violation() {
        let source = fixture("t001_violation.py");
        let extractor = PythonExtractor::new();
        let funcs = extractor.extract_test_functions(&source, "t001_violation.py");
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].analysis.assertion_count, 0);
    }

    #[test]
    fn assertion_count_positive_for_pass() {
        let source = fixture("t001_pass.py");
        let extractor = PythonExtractor::new();
        let funcs = extractor.extract_test_functions(&source, "t001_pass.py");
        assert_eq!(funcs[0].analysis.assertion_count, 1);
    }

    #[test]
    fn unittest_self_assert_counted() {
        let source = fixture("unittest_style.py");
        let extractor = PythonExtractor::new();
        let funcs = extractor.extract_test_functions(&source, "unittest_style.py");
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].analysis.assertion_count, 2);
    }

    // --- Cycle 3: Mock detection ---

    #[test]
    fn mock_count_for_violation() {
        let source = fixture("t002_violation.py");
        let extractor = PythonExtractor::new();
        let funcs = extractor.extract_test_functions(&source, "t002_violation.py");
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].analysis.mock_count, 6);
    }

    #[test]
    fn mock_count_for_pass() {
        let source = fixture("t002_pass.py");
        let extractor = PythonExtractor::new();
        let funcs = extractor.extract_test_functions(&source, "t002_pass.py");
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].analysis.mock_count, 1);
        assert_eq!(funcs[0].analysis.mock_classes, vec!["db"]);
    }

    #[test]
    fn mock_class_name_extraction() {
        assert_eq!(extract_mock_class_name("mock_db"), "db");
        assert_eq!(
            extract_mock_class_name("mock_payment_service"),
            "payment_service"
        );
        assert_eq!(extract_mock_class_name("my_mock"), "my_mock");
    }

    // --- Giant test ---

    #[test]
    fn giant_test_line_count() {
        let source = fixture("t003_violation.py");
        let extractor = PythonExtractor::new();
        let funcs = extractor.extract_test_functions(&source, "t003_violation.py");
        assert_eq!(funcs.len(), 1);
        assert!(funcs[0].analysis.line_count > 50);
    }

    #[test]
    fn short_test_line_count() {
        let source = fixture("t003_pass.py");
        let extractor = PythonExtractor::new();
        let funcs = extractor.extract_test_functions(&source, "t003_pass.py");
        assert_eq!(funcs.len(), 1);
        assert!(funcs[0].analysis.line_count <= 50);
    }

    // --- Phase 1 preserved tests ---

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
