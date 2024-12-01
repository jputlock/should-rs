use crate::{assertions::assert_comparison, context::AssertionContextBuilder, message_generator};

use std::fmt::Debug;

pub trait ShouldBePartialOrdExtension: PartialOrd + Debug {
    fn should_be_lt(&self, expected: &Self);
    fn should_be_leq(&self, expected: &Self);
    fn should_be_gt(&self, expected: &Self);
    fn should_be_geq(&self, expected: &Self);
}

impl<T> ShouldBePartialOrdExtension for T
where
    T: PartialOrd + Debug,
{
    fn should_be_lt(&self, expected: &Self) {
        assert_comparison(
            self,
            |x| x < expected,
            self,
            expected,
            AssertionContextBuilder::new().verb("should be less than"),
            message_generator::generate_message,
        );
    }

    fn should_be_leq(&self, expected: &Self) {
        assert_comparison(
            self,
            |x| x <= expected,
            &self,
            expected,
            AssertionContextBuilder::new().verb("should be less than or equal to"),
            message_generator::generate_message,
        );
    }

    fn should_be_gt(&self, expected: &Self) {
        assert_comparison(
            self,
            |x| x > expected,
            &self,
            expected,
            AssertionContextBuilder::new().verb("should be greater than"),
            message_generator::generate_message,
        );
    }

    fn should_be_geq(&self, expected: &Self) {
        assert_comparison(
            self,
            |x| x >= expected,
            &self,
            expected,
            AssertionContextBuilder::new().verb("should be greater than or equal to"),
            message_generator::generate_message,
        );
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
