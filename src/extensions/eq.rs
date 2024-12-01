use crate::{assertions::assert_comparison, context::AssertionContextBuilder, message_generator};

use std::fmt::Debug;

pub trait ShouldBeEqExtension: Eq + Debug {
    /// Assert that this object is equivalent to the given 'expected'.
    fn should_be_eq(&self, expected: &Self);

    /// Assert that this object is not equivalent to the given 'expected'.
    fn should_not_be_eq(&self, expected: &Self);
}

impl<T> ShouldBeEqExtension for T
where
    T: Eq + Debug,
{
    fn should_be_eq(&self, expected: &Self) {
        assert_comparison(
            self,
            |x| x == expected,
            &self,
            expected,
            AssertionContextBuilder::new(),
            message_generator::expected_vs_actual_message,
        );
    }

    fn should_not_be_eq(&self, expected: &Self) {
        assert_comparison(
            self,
            |x| x != expected,
            &self,
            expected,
            AssertionContextBuilder::new().actual_mapper(Box::new(|_| "".to_string())),
            message_generator::expected_vs_actual_message,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone)]
    struct Object {
        internal_string: String,
        internal_u32: u32,
    }

    impl Object {
        pub fn new(some_string: &str, some_u32: u32) -> Self {
            Object {
                internal_string: some_string.to_string(),
                internal_u32: some_u32,
            }
        }
    }

    #[test]
    fn should_be_basic() {
        let original = Object::new("object1", 1111);
        let cloned_original = original.clone();

        original.should_be_eq(&cloned_original);
        cloned_original.should_be_eq(&original);
    }

    #[test]
    fn should_not_be_basic() {
        let original = Object::new("object1", 1111);
        let unique = Object::new("object2", 2222);

        original.should_not_be_eq(&unique);
        unique.should_not_be_eq(&original);
    }

    #[test]
    fn failure() {
        let original = Object::new("object1", 1111);
        let unique = Object::new("object2", 2222);

        let result = std::panic::catch_unwind(|| original.should_be_eq(&unique));
        assert!(result.is_err());
    }
}
