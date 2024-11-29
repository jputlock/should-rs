use crate::{
    assertions::assert,
    context::{AssertionContext, AssertionContextBuilder},
    message_generator::ErrorMessageGenerator,
};

use std::fmt::Debug;

pub trait ShouldBePartialOrdExtension: PartialOrd + Debug {
    fn should_be_lt(&self, expected: &Self) {
        assert(
            |x| x < expected,
            self,
            expected,
            AssertionContextBuilder::new().verb("should be less than".to_string()),
            PartialOrdErrorMessageGenerator::generate_message,
        );
    }

    fn should_be_leq(&self, expected: &Self) {
        assert(
            |x| x <= expected,
            self,
            expected,
            AssertionContextBuilder::new().verb("should be less than or equal to".to_string()),
            PartialOrdErrorMessageGenerator::generate_message,
        );
    }

    fn should_be_gt(&self, expected: &Self) {
        assert(
            |x| x > expected,
            self,
            expected,
            AssertionContextBuilder::new().verb("should be greater than".to_string()),
            PartialOrdErrorMessageGenerator::generate_message,
        );
    }

    fn should_be_geq(&self, expected: &Self) {
        assert(
            |x| x >= expected,
            self,
            expected,
            AssertionContextBuilder::new().verb("should be greater than or equal to".to_string()),
            PartialOrdErrorMessageGenerator::generate_message,
        );
    }
}

impl<T> ShouldBePartialOrdExtension for T where T: PartialOrd + Debug {}

pub(crate) struct PartialOrdErrorMessageGenerator {}
impl ErrorMessageGenerator for PartialOrdErrorMessageGenerator {
    fn generate_message<T: ?Sized + Debug, O: ?Sized + Debug>(
        actual: &T,
        expected: &O,
        context: &AssertionContext,
    ) -> String {
        format!(
            "{} {} {expected:?} but was {actual:?}",
            context.asserted_expression, context.verb
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_comparisons() {
        for first in 1..=3 {
            for second in 1..=3 {
                if first < second {
                    first.should_be_lt(&second);
                    second.should_be_gt(&first);
                } else {
                    first.should_be_geq(&second);
                    second.should_be_leq(&first);
                }
            }
        }
    }

    #[test]
    fn failure() {
        let result = std::panic::catch_unwind(|| 1f32.should_be_geq(&2f32));
        assert!(result.is_err());
    }
}
