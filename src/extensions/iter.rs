use crate::assertions::{assert_comparison, assert_unary};
use crate::context::AssertionContextBuilder;
use crate::message_generator;

use std::fmt::Debug;

pub trait ShouldBeIntoIterExtension: IntoIterator<Item: Eq + Debug> + Clone + Debug {
    /// Assert that the generated sequence is a contiguous ordered slice of the
    /// given 'supersequence'.
    fn should_be_subsequence_of(self, supersequence: impl IntoIterator<Item = Self::Item> + Debug);

    /// Assert that the generated sequence consists of the elements in the given
    /// 'superset', ignoring order.
    fn should_be_subset_of(self, superset: impl IntoIterator<Item = Self::Item> + Debug);

    /// Assert that the generated sequence is empty.
    fn should_be_empty(self);

    /// Assert that the generated sequence contains at least one element.
    fn should_not_be_empty(self);

    /// Assert that the generated sequence of the given 'size'.
    fn should_be_size(self, size: usize);

    /// Assert that the generated sequence consists of unique items.
    // fn should_all_be_unique(self);

    /// Assert that the generated sequence contains the specified 'item'.
    fn should_contain(self, item: &Self::Item);

    /// Assert that the generated sequence does not contain the specified 'item'.
    fn should_not_contain(self, item: &Self::Item);

    /// Assert that the generated sequence has at least one element which
    /// satisfies the given 'matcher'. In other words, the given 'matcher' must
    /// return 'true' for at least one element in the generated sequence.
    fn should_any_satisfy(self, matcher: impl FnMut(Self::Item) -> bool);

    /// Assert that all elements in the generated sequence satisfy the given
    /// 'matcher'. In other words, the given 'matcher' must return 'true' for
    /// all elements in the generated sequence.
    fn should_all_satisfy(self, matcher: impl FnMut(Self::Item) -> bool);
}

impl<T> ShouldBeIntoIterExtension for T
where
    T: Iterator<Item: Eq + Debug> + Clone + Debug,
{
    fn should_be_subsequence_of(
        self,
        _supersequence: impl IntoIterator<Item = Self::Item> + Debug,
    ) {
        todo!()
    }

    fn should_be_subset_of(self, _superset: impl IntoIterator<Item = Self::Item> + Debug) {
        todo!()
    }

    fn should_be_empty(self) {
        let cloned = self.clone();

        assert_unary(
            self,
            |mut iter| iter.next().is_none(),
            cloned,
            "empty",
            AssertionContextBuilder::new(),
            message_generator::generate_message,
        );
    }

    fn should_not_be_empty(self) {
        let cloned = self.clone();

        assert_unary(
            self,
            |mut iter| iter.next().is_some(),
            cloned,
            "empty",
            AssertionContextBuilder::new().verb("should not be"),
            message_generator::generate_message,
        );
    }

    fn should_be_size(self, size: usize) {
        let cloned = self.clone();

        assert_unary(
            self,
            |iter| {
                let mut count: usize = 0;
                for _ in iter {
                    count += 1;
                }
                count == size
            },
            cloned,
            format!("size {size}").as_str(),
            AssertionContextBuilder::new().actual_mapper(Box::new(|iter: T| {
                let mut count: usize = 0;
                for _ in iter.clone() {
                    count += 1;
                }
                format!(" size {count}: {iter:?}")
            })),
            message_generator::generate_message,
        );
    }

    fn should_contain(self, item: &Self::Item) {
        let cloned = self.clone();

        assert_comparison(
            self,
            |mut iter: Self| iter.any(|x| x == *item),
            cloned,
            item,
            AssertionContextBuilder::new().verb("should contain"),
            message_generator::generate_message,
        );
    }

    fn should_not_contain(self, item: &Self::Item) {
        let cloned = self.clone();

        assert_comparison(
            self,
            |mut iter: Self| !iter.any(|x| x == *item),
            cloned,
            item,
            AssertionContextBuilder::new().verb("should not contain"),
            message_generator::generate_message,
        );
    }

    fn should_any_satisfy(self, predicate: impl FnMut(Self::Item) -> bool) {
        let cloned = self.clone();

        assert_unary(
            self,
            |mut iter: Self| iter.any(predicate),
            cloned,
            "at least one element which satsified the predicate",
            AssertionContextBuilder::new().verb("should have"),
            message_generator::generate_message,
        );
    }

    fn should_all_satisfy(self, predicate: impl FnMut(Self::Item) -> bool) {
        let cloned = self.clone();

        assert_unary(
            self,
            |mut iter: Self| iter.all(predicate),
            cloned,
            "all elements which satisfy the predicate",
            AssertionContextBuilder::new().verb("should have"),
            message_generator::generate_message,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_be_empty() {
        std::iter::empty::<i32>().should_be_empty();

        let result = std::panic::catch_unwind(|| (0..3).should_be_empty());
        assert!(result.is_err());
    }

    #[test]
    fn test_should_not_be_empty() {
        (0..3).should_not_be_empty();

        let result = std::panic::catch_unwind(|| std::iter::empty::<i32>().should_not_be_empty());
        assert!(result.is_err());
    }

    #[test]
    fn test_should_be_size() {
        (0..3).should_be_size(3);

        let result = std::panic::catch_unwind(|| (0..3).should_be_size(2));
        assert!(result.is_err());

        let result = std::panic::catch_unwind(|| (0..3).should_be_size(4));
        assert!(result.is_err());
    }

    #[test]
    fn test_should_contain() {
        (0..3).should_contain(&2);

        let result = std::panic::catch_unwind(|| (0..3).should_contain(&10));
        assert!(result.is_err());
    }

    #[test]
    fn test_should_not_contain() {
        (0..3).should_not_contain(&10);

        let result = std::panic::catch_unwind(|| (0..3).should_not_contain(&2));
        assert!(result.is_err());
    }

    #[test]
    fn test_should_any_satisfy() {
        (0..3).should_any_satisfy(|x| x == 0);

        let result = std::panic::catch_unwind(|| (0..3).should_any_satisfy(|x| x == 10));
        assert!(result.is_err());
    }

    #[test]
    fn test_should_all_satisfy() {
        (0..3).should_all_satisfy(|x| x < 5);

        let result = std::panic::catch_unwind(|| (0..3).should_all_satisfy(|x| x < 1));
        assert!(result.is_err());
    }
}
