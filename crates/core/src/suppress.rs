use crate::rules::RuleId;

pub fn parse_suppression(comment: &str) -> Vec<RuleId> {
    let prefix = "exspec-ignore:";
    if let Some(rest) = comment.find(prefix).map(|i| &comment[i + prefix.len()..]) {
        rest.split(',').map(|s| RuleId::new(s.trim())).collect()
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_rule() {
        let result = parse_suppression("# exspec-ignore: T002");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "T002");
    }

    #[test]
    fn parse_multiple_rules() {
        let result = parse_suppression("# exspec-ignore: T002, T003");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, "T002");
        assert_eq!(result[1].0, "T003");
    }

    #[test]
    fn parse_no_suppression() {
        let result = parse_suppression("# just a comment");
        assert!(result.is_empty());
    }
}
