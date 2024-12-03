use crate::{
    assertions::{assert_comparison, assert_unary},
    context::AssertionContextBuilder,
    message_generator,
};

use std::fmt::Debug;

pub trait ShouldBeStringExtension: AsRef<str> + Debug {
    /// Assert that this string should be empty.
    fn should_be_empty(&self) {
        assert_unary(
            self.as_ref(),
            |x| x.is_empty(),
            self.as_ref(),
            AssertionContextBuilder::new().verb("should be empty"),
            message_generator::failed_condition_message,
        );
    }

    /// Assert that this string should not be empty.
    fn should_not_be_empty(&self) {
        assert_unary(
            self.as_ref(),
            |x| !x.is_empty(),
            self.as_ref(),
            AssertionContextBuilder::new().verb("should be empty"),
            message_generator::failed_condition_message,
        );
    }

    /// Assert that this string should be the given 'length'.
    fn should_be_len(&self, length: usize) {
        assert_comparison(
            self.as_ref(),
            |x| x.len() == length,
            self.as_ref(),
            length,
            AssertionContextBuilder::new()
                .verb(format!("should be length {length}").as_str())
                .actual_mapper(Box::new(|x: &str| {
                    format!(" length {}, actual={x}", x.len())
                })),
            message_generator::expected_vs_actual_message,
        );
    }

    /// Assert that this string contains the given 'pattern'.
    // Eventually would like to swap arg to [`std::str::pattern::Pattern`], see
    // https://github.com/rust-lang/rust/issues/27721
    fn should_contain(&self, pattern: &str) {
        assert_comparison(
            self.as_ref(),
            |x| x.contains(pattern),
            self.as_ref(),
            pattern,
            AssertionContextBuilder::new().verb("should contain"),
            message_generator::expected_vs_actual_message,
        );
    }

    /// Assert that this string does not contain the given 'pattern'.
    // Eventually would like to swap arg to [`std::str::pattern::Pattern`], see
    // https://github.com/rust-lang/rust/issues/27721
    fn should_not_contain(&self, pattern: &str) {
        assert_comparison(
            self.as_ref(),
            |x| !x.contains(pattern),
            self.as_ref(),
            pattern,
            AssertionContextBuilder::new().verb("should not contain"),
            message_generator::expected_vs_actual_message,
        );
    }
}

impl<T> ShouldBeStringExtension for T where T: AsRef<str> + Debug {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_should_be_empty() {
        "".should_be_empty();

        let result = std::panic::catch_unwind(|| "nonempty".should_be_empty());
        assert!(result.is_err());
    }

    #[test]
    fn test_str_should_not_be_empty() {
        "nonempty".should_not_be_empty();

        let result = std::panic::catch_unwind(|| "".should_not_be_empty());
        assert!(result.is_err());
    }

    #[test]
    fn test_str_should_be_len() {
        "hello".should_be_len(5);

        let result = std::panic::catch_unwind(|| "hello".should_be_len(4));
        assert!(result.is_err());
    }

    #[test]
    fn test_str_should_contain() {
        "hello world".should_contain("hello");

        let result = std::panic::catch_unwind(|| "hello".should_contain("world"));
        assert!(result.is_err());
    }

    #[test]
    fn test_str_should_not_contain() {
        "hello".should_not_contain("world");

        let result = std::panic::catch_unwind(|| "hello world".should_not_contain("world"));
        assert!(result.is_err());
    }

    #[test]
    fn test_string_should_be_empty() {
        "".to_string().should_be_empty();

        let result = std::panic::catch_unwind(|| "nonempty".to_string().should_be_empty());
        assert!(result.is_err());
    }

    #[test]
    fn test_string_should_not_be_empty() {
        "nonempty".to_string().should_not_be_empty();

        let result = std::panic::catch_unwind(|| "".to_string().should_not_be_empty());
        assert!(result.is_err());
    }

    #[test]
    fn test_string_should_be_len() {
        let hello = "hello".to_string();
        hello.should_be_len(5);

        let result = std::panic::catch_unwind(|| hello.should_be_len(4));
        assert!(result.is_err());
    }

    #[test]
    fn test_string_should_contain() {
        let hello = "hello".to_string();
        hello.should_contain(&hello[0..2]);

        let result = std::panic::catch_unwind(|| hello.should_contain("world"));
        assert!(result.is_err());
    }

    #[test]
    fn test_string_should_not_contain() {
        let hello = "hello".to_string();
        hello.should_not_contain("world");

        let result = std::panic::catch_unwind(|| hello.should_not_contain(&hello[0..2]));
        assert!(result.is_err());
    }
}
