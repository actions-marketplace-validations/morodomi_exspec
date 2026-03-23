// Fixture: attribute containing "should_panic" as substring should NOT count as assertion.
// #[my_crate::should_panic_handler] is not the same as #[should_panic].

#[cfg(test)]
mod tests {
    #[test]
    #[my_should_panic_wrapper]
    fn test_with_similar_attr_name() {
        // This test has NO real #[should_panic] attribute.
        // The attribute name merely contains "should_panic" as substring.
        // T001 should fire (assertion_count == 0).
    }
}
