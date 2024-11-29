use crate::{
    assertions::assert,
    context::{AssertionContext, AssertionContextBuilder},
    message_generator::ErrorMessageGenerator,
};

use std::{fmt::Debug, iter::zip};

pub trait ShouldBeIterExtension: Iterator<Item: Eq + Debug> + Debug {
    fn should_be(&mut self, expected: impl IntoIterator<Item = Self::Item> + Debug) {
        let expected = expected.into();

        assert(
            |x| zip(self, expected).all(|(x, y)| x == y),
            &self,
            &expected,
            AssertionContextBuilder::new(),
            IterErrorMessageGenerator::generate_message,
        );
    }
}

impl<T> ShouldBeIterExtension for T where T: Iterator<Item: Eq + Debug> + Debug {}

pub(crate) struct IterErrorMessageGenerator {}
impl ErrorMessageGenerator for IterErrorMessageGenerator {
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
    use super::ShouldBeIterExtension;

    #[test]
    fn basic() {
        let my_vec = vec![1, 2, 3];

        my_vec.iter().should_be((1..=3).i);
    }
}
