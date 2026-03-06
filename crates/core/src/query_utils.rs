use std::collections::BTreeSet;

use streaming_iterator::StreamingIterator;
use tree_sitter::{Node, Query, QueryCursor};

use crate::rules::RuleId;
use crate::suppress::parse_suppression;

pub fn count_captures(query: &Query, capture_name: &str, node: Node, source: &[u8]) -> usize {
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
    let var_idx = query
        .capture_index_for_name("var_name")
        .expect("no @var_name capture");
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
}
