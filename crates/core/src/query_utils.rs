use std::collections::{BTreeSet, HashMap};

use streaming_iterator::StreamingIterator;
use tree_sitter::{Node, Query, QueryCursor, Tree};

use crate::rules::RuleId;
use crate::suppress::parse_suppression;

pub fn count_captures(query: &Query, capture_name: &str, node: Node, source: &[u8]) -> usize {
    let idx = match query.capture_index_for_name(capture_name) {
        Some(i) => i,
        None => return 0,
    };
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, node, source);
    let mut count = 0;
    while let Some(m) = matches.next() {
        count += m.captures.iter().filter(|c| c.index == idx).count();
    }
    count
}

pub fn has_any_match(query: &Query, capture_name: &str, node: Node, source: &[u8]) -> bool {
    let idx = match query.capture_index_for_name(capture_name) {
        Some(i) => i,
        None => return false,
    };
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, node, source);
    while let Some(m) = matches.next() {
        if m.captures.iter().any(|c| c.index == idx) {
            return true;
        }
    }
    false
}

pub fn collect_mock_class_names<F>(
    query: &Query,
    node: Node,
    source: &[u8],
    extract_name: F,
) -> Vec<String>
where
    F: Fn(&str) -> String,
{
    let var_idx = match query.capture_index_for_name("var_name") {
        Some(i) => i,
        None => return Vec::new(),
    };
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, node, source);
    let mut names = BTreeSet::new();
    while let Some(m) = matches.next() {
        for c in m.captures.iter().filter(|c| c.index == var_idx) {
            if let Ok(var) = c.node.utf8_text(source) {
                names.insert(extract_name(var));
            }
        }
    }
    names.into_iter().collect()
}

/// Collect byte ranges of all captures matching `capture_name` in `query`.
fn collect_capture_ranges(
    query: &Query,
    capture_name: &str,
    node: Node,
    source: &[u8],
) -> Vec<(usize, usize)> {
    let idx = match query.capture_index_for_name(capture_name) {
        Some(i) => i,
        None => return Vec::new(),
    };
    let mut ranges = Vec::new();
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, node, source);
    while let Some(m) = matches.next() {
        for c in m.captures.iter().filter(|c| c.index == idx) {
            ranges.push((c.node.start_byte(), c.node.end_byte()));
        }
    }
    ranges
}

/// Count captures of `inner_capture` from `inner_query` that fall within
/// byte ranges of `outer_capture` from `outer_query`.
pub fn count_captures_within_context(
    outer_query: &Query,
    outer_capture: &str,
    inner_query: &Query,
    inner_capture: &str,
    node: Node,
    source: &[u8],
) -> usize {
    let ranges = collect_capture_ranges(outer_query, outer_capture, node, source);
    if ranges.is_empty() {
        return 0;
    }

    let inner_idx = match inner_query.capture_index_for_name(inner_capture) {
        Some(i) => i,
        None => return 0,
    };

    let mut count = 0;
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(inner_query, node, source);
    while let Some(m) = matches.next() {
        for c in m.captures.iter().filter(|c| c.index == inner_idx) {
            let start = c.node.start_byte();
            let end = c.node.end_byte();
            if ranges.iter().any(|(rs, re)| start >= *rs && end <= *re) {
                count += 1;
            }
        }
    }

    count
}

// Literals considered too common to flag as duplicates.
// Cross-language superset: Python (True/False/None), JS (null/undefined), PHP/Ruby (nil).
const TRIVIAL_LITERALS: &[&str] = &[
    "0",
    "1",
    "2",
    "true",
    "false",
    "True",
    "False",
    "None",
    "null",
    "undefined",
    "nil",
    "\"\"",
    "''",
    "0.0",
    "1.0",
];

/// Count the maximum number of times any non-trivial literal appears
/// within assertion nodes of the given function node.
///
/// `assertion_query` must have an `@assertion` capture.
/// `literal_kinds` lists the tree-sitter node kind names that represent literals
/// for the target language (e.g., `["integer", "float", "string"]` for Python).
pub fn count_duplicate_literals(
    assertion_query: &Query,
    node: Node,
    source: &[u8],
    literal_kinds: &[&str],
) -> usize {
    let ranges = collect_capture_ranges(assertion_query, "assertion", node, source);
    if ranges.is_empty() {
        return 0;
    }

    // Walk tree, collect literals within assertion ranges
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut stack = vec![node];
    while let Some(n) = stack.pop() {
        let start = n.start_byte();
        let end = n.end_byte();

        // Prune subtrees that don't overlap with any assertion range
        let overlaps_any = ranges.iter().any(|(rs, re)| end > *rs && start < *re);
        if !overlaps_any {
            continue;
        }

        if literal_kinds.contains(&n.kind()) {
            let in_assertion = ranges.iter().any(|(rs, re)| start >= *rs && end <= *re);
            if in_assertion {
                if let Ok(text) = n.utf8_text(source) {
                    if !TRIVIAL_LITERALS.contains(&text) {
                        *counts.entry(text.to_string()).or_insert(0) += 1;
                    }
                }
            }
        }

        for i in 0..n.child_count() {
            if let Some(child) = n.child(i) {
                stack.push(child);
            }
        }
    }

    counts.values().copied().max().unwrap_or(0)
}

/// Text-based fallback for T001 escape hatch. Patterns are literal substrings, not regex.
/// Matches in comments, strings, and imports are included by design.
/// Returns the number of source lines that contain any pattern as a substring.
pub fn count_custom_assertion_lines(source_lines: &[&str], patterns: &[String]) -> usize {
    if patterns.is_empty() {
        return 0;
    }
    source_lines
        .iter()
        .filter(|line| {
            patterns
                .iter()
                .any(|p| !p.is_empty() && line.contains(p.as_str()))
        })
        .count()
}

/// Apply custom assertion pattern fallback to functions with assertion_count == 0.
/// Only functions with no detected assertions are augmented; others are untouched.
pub fn apply_custom_assertion_fallback(
    analysis: &mut crate::extractor::FileAnalysis,
    source: &str,
    patterns: &[String],
) {
    if patterns.is_empty() {
        return;
    }
    let lines: Vec<&str> = source.lines().collect();
    for func in &mut analysis.functions {
        if func.analysis.assertion_count > 0 {
            continue;
        }
        // line/end_line are 1-based
        let start = func.line.saturating_sub(1);
        let end = func.end_line.min(lines.len());
        if start >= end {
            continue;
        }
        let body_lines = &lines[start..end];
        let count = count_custom_assertion_lines(body_lines, patterns);
        func.analysis.assertion_count += count;
    }
}

/// Apply same-file helper tracing to augment assertion_count for functions with 0 assertions.
///
/// For each test function with assertion_count == 0, traces 1-hop function calls within the
/// same file. If a called function's body contains assertions, those assertions are counted
/// and added to the test function's assertion_count.
///
/// - Only 1-hop: calls from test → helper (not helper → helper)
/// - Only functions with assertion_count == 0 are processed (early return for performance)
/// - Missing/undefined called functions are silently ignored (no crash)
///
/// `call_query`: tree-sitter query with @call_name capture
/// `def_query`: tree-sitter query with @def_name and @def_body captures
/// `assertion_query`: language assertion query with @assertion capture
pub fn apply_same_file_helper_tracing(
    analysis: &mut crate::extractor::FileAnalysis,
    tree: &Tree,
    source: &[u8],
    call_query: &Query,
    def_query: &Query,
    assertion_query: &Query,
) {
    // Early return: no assertion-free functions → nothing to trace
    if !analysis
        .functions
        .iter()
        .any(|f| f.analysis.assertion_count == 0)
    {
        return;
    }

    let root = tree.root_node();

    // Step 1: Build helper definition map: name → body byte range
    let def_name_idx = match def_query.capture_index_for_name("def_name") {
        Some(i) => i,
        None => return,
    };
    let def_body_idx = match def_query.capture_index_for_name("def_body") {
        Some(i) => i,
        None => return,
    };

    let mut helper_bodies: HashMap<String, (usize, usize)> = HashMap::new();
    {
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(def_query, root, source);
        while let Some(m) = matches.next() {
            let mut name: Option<String> = None;
            let mut body_range: Option<(usize, usize)> = None;
            for cap in m.captures {
                if cap.index == def_name_idx {
                    name = cap.node.utf8_text(source).ok().map(|s| s.to_string());
                } else if cap.index == def_body_idx {
                    body_range = Some((cap.node.start_byte(), cap.node.end_byte()));
                }
            }
            if let (Some(n), Some(r)) = (name, body_range) {
                helper_bodies.insert(n, r);
            }
        }
    }

    if helper_bodies.is_empty() {
        return;
    }

    // Step 2: Build line-to-byte-offset map
    let line_starts: Vec<usize> =
        std::iter::once(0)
            .chain(source.iter().enumerate().filter_map(|(i, &b)| {
                if b == b'\n' {
                    Some(i + 1)
                } else {
                    None
                }
            }))
            .collect();

    // Step 3: For each assertion-free test function, trace helper calls
    let call_name_idx = match call_query.capture_index_for_name("call_name") {
        Some(i) => i,
        None => return,
    };

    for func in &mut analysis.functions {
        if func.analysis.assertion_count > 0 {
            continue;
        }

        // Calculate byte range from 1-based line numbers
        let start_byte = line_starts
            .get(func.line.saturating_sub(1))
            .copied()
            .unwrap_or(0);
        let end_byte = line_starts
            .get(func.end_line.min(line_starts.len()))
            .copied()
            .unwrap_or(source.len());

        // Collect called function names within this test function's byte range
        let mut called_names: BTreeSet<String> = BTreeSet::new();
        {
            let mut call_cursor = QueryCursor::new();
            call_cursor.set_byte_range(start_byte..end_byte);
            let mut call_matches = call_cursor.matches(call_query, root, source);
            while let Some(m) = call_matches.next() {
                for cap in m.captures {
                    if cap.index == call_name_idx {
                        if let Ok(name) = cap.node.utf8_text(source) {
                            called_names.insert(name.to_string());
                        }
                    }
                }
            }
        }

        // For each unique called name, look up its body and count assertions
        let mut traced_count = 0usize;
        for name in &called_names {
            if let Some(&(body_start, body_end)) = helper_bodies.get(name.as_str()) {
                // Find the body node from the tree by byte range
                if let Some(body_node) = root.descendant_for_byte_range(body_start, body_end) {
                    traced_count += count_captures(assertion_query, "assertion", body_node, source);
                }
            }
        }

        func.analysis.assertion_count += traced_count;
    }
}

pub fn extract_suppression_from_previous_line(source: &str, start_row: usize) -> Vec<RuleId> {
    if start_row == 0 {
        return Vec::new();
    }
    let lines: Vec<&str> = source.lines().collect();
    let prev_line = lines.get(start_row - 1).unwrap_or(&"");
    parse_suppression(prev_line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suppression_from_first_line_returns_empty() {
        assert!(extract_suppression_from_previous_line("any source", 0).is_empty());
    }

    #[test]
    fn suppression_from_previous_line_parses_comment() {
        let source = "// exspec-ignore: T001\nfn test_foo() {}";
        let result = extract_suppression_from_previous_line(source, 1);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "T001");
    }

    #[test]
    fn suppression_from_previous_line_no_comment() {
        let source = "// normal comment\nfn test_foo() {}";
        let result = extract_suppression_from_previous_line(source, 1);
        assert!(result.is_empty());
    }

    #[test]
    fn suppression_out_of_bounds_returns_empty() {
        let source = "single line";
        let result = extract_suppression_from_previous_line(source, 5);
        assert!(result.is_empty());
    }

    // --- count_captures_within_context ---

    fn python_language() -> tree_sitter::Language {
        tree_sitter_python::LANGUAGE.into()
    }

    #[test]
    fn count_captures_within_context_basic() {
        // assert obj._count == 1 -> _count is inside assert_statement (@assertion)
        let source = "def test_foo():\n    assert obj._count == 1\n";
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&python_language()).unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        let assertion_query =
            Query::new(&python_language(), "(assert_statement) @assertion").unwrap();
        let private_query = Query::new(
            &python_language(),
            "(attribute attribute: (identifier) @private_access (#match? @private_access \"^_[^_]\"))",
        )
        .unwrap();

        let count = count_captures_within_context(
            &assertion_query,
            "assertion",
            &private_query,
            "private_access",
            root,
            source.as_bytes(),
        );
        assert_eq!(count, 1, "should detect _count inside assert statement");
    }

    #[test]
    fn count_captures_within_context_outside() {
        // _count is outside assert -> should not count
        let source = "def test_foo():\n    x = obj._count\n    assert x == 1\n";
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&python_language()).unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        let assertion_query =
            Query::new(&python_language(), "(assert_statement) @assertion").unwrap();
        let private_query = Query::new(
            &python_language(),
            "(attribute attribute: (identifier) @private_access (#match? @private_access \"^_[^_]\"))",
        )
        .unwrap();

        let count = count_captures_within_context(
            &assertion_query,
            "assertion",
            &private_query,
            "private_access",
            root,
            source.as_bytes(),
        );
        assert_eq!(count, 0, "_count is outside assert, should not count");
    }

    #[test]
    fn count_captures_within_context_no_outer() {
        // No assert statement at all
        let source = "def test_foo():\n    x = obj._count\n";
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&python_language()).unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        let assertion_query =
            Query::new(&python_language(), "(assert_statement) @assertion").unwrap();
        let private_query = Query::new(
            &python_language(),
            "(attribute attribute: (identifier) @private_access (#match? @private_access \"^_[^_]\"))",
        )
        .unwrap();

        let count = count_captures_within_context(
            &assertion_query,
            "assertion",
            &private_query,
            "private_access",
            root,
            source.as_bytes(),
        );
        assert_eq!(count, 0, "no assertions, should return 0");
    }

    #[test]
    fn count_captures_missing_capture_returns_zero() {
        let lang = python_language();
        // Query with capture @assertion, but we ask for nonexistent name
        let query = Query::new(&lang, "(assert_statement) @assertion").unwrap();
        let source = "def test_foo():\n    assert True\n";
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&lang).unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        let count = count_captures(&query, "nonexistent", root, source.as_bytes());
        assert_eq!(count, 0, "missing capture name should return 0, not panic");
    }

    #[test]
    fn collect_mock_class_names_missing_capture_returns_empty() {
        let lang = python_language();
        // Query without @var_name capture
        let query = Query::new(&lang, "(assert_statement) @assertion").unwrap();
        let source = "def test_foo():\n    assert True\n";
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&lang).unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        let names = collect_mock_class_names(&query, root, source.as_bytes(), |s| s.to_string());
        assert!(
            names.is_empty(),
            "missing @var_name capture should return empty vec, not panic"
        );
    }

    #[test]
    fn count_captures_within_context_missing_capture() {
        // Capture name doesn't exist in query -> defensive 0
        let source = "def test_foo():\n    assert obj._count == 1\n";
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&python_language()).unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        let assertion_query =
            Query::new(&python_language(), "(assert_statement) @assertion").unwrap();
        let private_query = Query::new(
            &python_language(),
            "(attribute attribute: (identifier) @private_access (#match? @private_access \"^_[^_]\"))",
        )
        .unwrap();

        // Wrong capture name for outer
        let count = count_captures_within_context(
            &assertion_query,
            "nonexistent",
            &private_query,
            "private_access",
            root,
            source.as_bytes(),
        );
        assert_eq!(count, 0, "missing outer capture should return 0");

        // Wrong capture name for inner
        let count = count_captures_within_context(
            &assertion_query,
            "assertion",
            &private_query,
            "nonexistent",
            root,
            source.as_bytes(),
        );
        assert_eq!(count, 0, "missing inner capture should return 0");
    }

    // --- count_duplicate_literals ---

    #[test]
    fn count_duplicate_literals_detects_repeated_value() {
        let source = "def test_foo():\n    assert calc(1) == 42\n    assert calc(2) == 42\n    assert calc(3) == 42\n";
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&python_language()).unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        let assertion_query =
            Query::new(&python_language(), "(assert_statement) @assertion").unwrap();
        let count = count_duplicate_literals(
            &assertion_query,
            root,
            source.as_bytes(),
            &["integer", "float", "string"],
        );
        assert_eq!(count, 3, "42 appears 3 times in assertions");
    }

    #[test]
    fn count_duplicate_literals_trivial_excluded() {
        // All literals are trivial (0, 1, 2) - should return 0
        let source =
            "def test_foo():\n    assert calc(1) == 0\n    assert calc(2) == 0\n    assert calc(1) == 0\n";
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&python_language()).unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        let assertion_query =
            Query::new(&python_language(), "(assert_statement) @assertion").unwrap();
        let count = count_duplicate_literals(
            &assertion_query,
            root,
            source.as_bytes(),
            &["integer", "float", "string"],
        );
        assert_eq!(count, 0, "0, 1, 2 are all trivial and should be excluded");
    }

    #[test]
    fn count_duplicate_literals_no_assertions() {
        let source = "def test_foo():\n    x = 42\n    y = 42\n    z = 42\n";
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&python_language()).unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        let assertion_query =
            Query::new(&python_language(), "(assert_statement) @assertion").unwrap();
        let count = count_duplicate_literals(
            &assertion_query,
            root,
            source.as_bytes(),
            &["integer", "float", "string"],
        );
        assert_eq!(count, 0, "no assertions, should return 0");
    }

    // --- count_custom_assertion_lines ---

    // TC-04: empty patterns -> 0
    #[test]
    fn count_custom_assertion_lines_empty_patterns() {
        let lines = vec!["util.assertEqual(x, 1)", "assert True"];
        assert_eq!(count_custom_assertion_lines(&lines, &[]), 0);
    }

    // TC-05: matching pattern returns correct count
    #[test]
    fn count_custom_assertion_lines_matching() {
        let lines = vec![
            "    util.assertEqual(x, 1)",
            "    util.assertEqual(y, 2)",
            "    print(result)",
        ];
        let patterns = vec!["util.assertEqual(".to_string()];
        assert_eq!(count_custom_assertion_lines(&lines, &patterns), 2);
    }

    // TC-06: pattern in comment still counts (by design)
    #[test]
    fn count_custom_assertion_lines_in_comment() {
        let lines = vec!["    # util.assertEqual(x, 1)", "    pass"];
        let patterns = vec!["util.assertEqual(".to_string()];
        assert_eq!(count_custom_assertion_lines(&lines, &patterns), 1);
    }

    // TC-07: no matches -> 0
    #[test]
    fn count_custom_assertion_lines_no_match() {
        let lines = vec!["    result = compute(42)", "    print(result)"];
        let patterns = vec!["util.assertEqual(".to_string()];
        assert_eq!(count_custom_assertion_lines(&lines, &patterns), 0);
    }

    // TC-08: same pattern on multiple lines returns line count
    #[test]
    fn count_custom_assertion_lines_multiple_occurrences() {
        let lines = vec!["    myAssert(a) and myAssert(b)", "    myAssert(c)"];
        let patterns = vec!["myAssert(".to_string()];
        // Line count, not occurrence count: line 1 has 2 but counts as 1
        assert_eq!(count_custom_assertion_lines(&lines, &patterns), 2);
    }

    // TC-16: multiple patterns, one matches
    #[test]
    fn count_custom_assertion_lines_multiple_patterns() {
        let lines = vec!["    customCheck(x)"];
        let patterns = vec!["util.assertEqual(".to_string(), "customCheck(".to_string()];
        assert_eq!(count_custom_assertion_lines(&lines, &patterns), 1);
    }

    // --- apply_custom_assertion_fallback ---

    // TC-09: assertion_count > 0 -> unchanged
    #[test]
    fn apply_fallback_skips_functions_with_assertions() {
        use crate::extractor::{FileAnalysis, TestAnalysis, TestFunction};

        let source = "def test_foo():\n    util.assertEqual(x, 1)\n    assert True\n";
        let mut analysis = FileAnalysis {
            file: "test.py".to_string(),
            functions: vec![TestFunction {
                name: "test_foo".to_string(),
                file: "test.py".to_string(),
                line: 1,
                end_line: 3,
                analysis: TestAnalysis {
                    assertion_count: 1,
                    ..Default::default()
                },
            }],
            has_pbt_import: false,
            has_contract_import: false,
            has_error_test: false,
            has_relational_assertion: false,
            parameterized_count: 0,
        };
        let patterns = vec!["util.assertEqual(".to_string()];
        apply_custom_assertion_fallback(&mut analysis, source, &patterns);
        assert_eq!(analysis.functions[0].analysis.assertion_count, 1);
    }

    // TC-10: assertion_count == 0 + custom match -> incremented
    #[test]
    fn apply_fallback_increments_assertion_count() {
        use crate::extractor::{FileAnalysis, TestAnalysis, TestFunction};

        let source = "def test_foo():\n    util.assertEqual(x, 1)\n    util.assertEqual(y, 2)\n";
        let mut analysis = FileAnalysis {
            file: "test.py".to_string(),
            functions: vec![TestFunction {
                name: "test_foo".to_string(),
                file: "test.py".to_string(),
                line: 1,
                end_line: 3,
                analysis: TestAnalysis {
                    assertion_count: 0,
                    ..Default::default()
                },
            }],
            has_pbt_import: false,
            has_contract_import: false,
            has_error_test: false,
            has_relational_assertion: false,
            parameterized_count: 0,
        };
        let patterns = vec!["util.assertEqual(".to_string()];
        apply_custom_assertion_fallback(&mut analysis, source, &patterns);
        assert_eq!(analysis.functions[0].analysis.assertion_count, 2);
    }

    // Empty patterns -> no-op
    #[test]
    fn apply_fallback_empty_patterns_noop() {
        use crate::extractor::{FileAnalysis, TestAnalysis, TestFunction};

        let source = "def test_foo():\n    util.assertEqual(x, 1)\n";
        let mut analysis = FileAnalysis {
            file: "test.py".to_string(),
            functions: vec![TestFunction {
                name: "test_foo".to_string(),
                file: "test.py".to_string(),
                line: 1,
                end_line: 2,
                analysis: TestAnalysis {
                    assertion_count: 0,
                    ..Default::default()
                },
            }],
            has_pbt_import: false,
            has_contract_import: false,
            has_error_test: false,
            has_relational_assertion: false,
            parameterized_count: 0,
        };
        apply_custom_assertion_fallback(&mut analysis, source, &[]);
        assert_eq!(analysis.functions[0].analysis.assertion_count, 0);
    }

    // --- empty string pattern filter ---

    #[test]
    fn empty_string_pattern_ignored() {
        let lines = vec!["assert True", "x = 1", "print(result)"];
        let patterns = vec!["".to_string()];
        assert_eq!(
            count_custom_assertion_lines(&lines, &patterns),
            0,
            "empty string pattern should not match any line"
        );
    }

    #[test]
    fn mixed_empty_and_valid_patterns() {
        let lines = vec!["    assert_custom(x)", "    print(result)"];
        let patterns = vec!["".to_string(), "assert_custom".to_string()];
        assert_eq!(
            count_custom_assertion_lines(&lines, &patterns),
            1,
            "only valid patterns should match"
        );
    }

    #[test]
    fn whitespace_only_pattern_matches() {
        // Whitespace-only patterns are NOT filtered (only empty string is)
        let lines = vec!["assert_true", "no_space_here"];
        let patterns = vec![" ".to_string()];
        assert_eq!(
            count_custom_assertion_lines(&lines, &patterns),
            0,
            "whitespace pattern should not match lines without spaces"
        );
        let lines_with_space = vec!["assert true", "nospace"];
        assert_eq!(
            count_custom_assertion_lines(&lines_with_space, &patterns),
            1,
            "whitespace pattern should match lines containing spaces"
        );
    }

    // --- apply_custom_assertion_fallback edge cases ---

    #[test]
    fn apply_fallback_end_line_exceeds_source() {
        use crate::extractor::{FileAnalysis, TestAnalysis, TestFunction};

        let source = "def test_foo():\n    custom_assert(x)\n";
        let mut analysis = FileAnalysis {
            file: "test.py".to_string(),
            functions: vec![TestFunction {
                name: "test_foo".to_string(),
                file: "test.py".to_string(),
                line: 1,
                end_line: 12, // well beyond source length (2 lines)
                analysis: TestAnalysis {
                    assertion_count: 0,
                    ..Default::default()
                },
            }],
            has_pbt_import: false,
            has_contract_import: false,
            has_error_test: false,
            has_relational_assertion: false,
            parameterized_count: 0,
        };
        let patterns = vec!["custom_assert".to_string()];
        apply_custom_assertion_fallback(&mut analysis, source, &patterns);
        assert_eq!(
            analysis.functions[0].analysis.assertion_count, 1,
            "should handle end_line > source length without panic"
        );
    }

    #[test]
    fn apply_fallback_empty_string_pattern_noop() {
        use crate::extractor::{FileAnalysis, TestAnalysis, TestFunction};

        let source = "def test_foo():\n    some_call(x)\n    another_call(y)\n";
        let mut analysis = FileAnalysis {
            file: "test.py".to_string(),
            functions: vec![TestFunction {
                name: "test_foo".to_string(),
                file: "test.py".to_string(),
                line: 1,
                end_line: 3,
                analysis: TestAnalysis {
                    assertion_count: 0,
                    ..Default::default()
                },
            }],
            has_pbt_import: false,
            has_contract_import: false,
            has_error_test: false,
            has_relational_assertion: false,
            parameterized_count: 0,
        };
        let patterns = vec!["".to_string()];
        apply_custom_assertion_fallback(&mut analysis, source, &patterns);
        assert_eq!(
            analysis.functions[0].analysis.assertion_count, 0,
            "empty-string-only patterns should not increment assertion_count"
        );
    }

    #[test]
    fn count_duplicate_literals_missing_capture() {
        let source = "def test_foo():\n    assert 42 == 42\n";
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&python_language()).unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        // Query without @assertion capture
        let query = Query::new(&python_language(), "(assert_statement) @something_else").unwrap();
        let count = count_duplicate_literals(&query, root, source.as_bytes(), &["integer"]);
        assert_eq!(count, 0, "missing @assertion capture should return 0");
    }
}
