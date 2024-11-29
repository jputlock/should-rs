use crate::{
    assertions::assert,
    context::{AssertionContext, AssertionContextBuilder},
    message_generator::ErrorMessageGenerator,
};

use std::fmt::Debug;

pub trait ShouldBeEqExtension: Eq + Debug {
    fn should_be_eq(&self, expected: &Self) {
        assert(
            |x| x == expected,
            self,
            expected,
            AssertionContextBuilder::new(),
            EqErrorMessageGenerator::generate_message,
        );
    }

    fn should_not_be_eq(&self, expected: &Self) {
        assert(
            |x| x != expected,
            self,
            expected,
            AssertionContextBuilder::new().is_negated(true),
            EqErrorMessageGenerator::generate_message,
        );
    }
}

impl<T> ShouldBeEqExtension for T where T: Eq + Debug {}

pub(crate) struct EqErrorMessageGenerator {}
impl ErrorMessageGenerator for EqErrorMessageGenerator {
    fn generate_message<T: ?Sized + Debug, O: ?Sized + Debug>(
        actual: &T,
        expected: &O,
        context: &AssertionContext,
    ) -> String {
        let value = if context.is_negated {
            "".to_string()
        } else {
            format!(" {:?}", actual)
        };

        format!(
            "{} {} {expected:?} but was{value}",
            context.asserted_expression, context.verb
        )
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
