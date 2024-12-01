use crate::{assertions::assert_comparison, context::AssertionContextBuilder, message_generator};

use std::fmt::Debug;

pub trait ShouldSatisfyExtension: Debug {
    /// Assert that this object satisfies the predicate. In other words, the
    /// predicate must return 'true' when evaluated with 'self'.
    ///
    /// More specialized functions should be preferred over this one.
    fn should_satisfy(&self, predicate: impl FnMut(&Self) -> bool, custom_message: Option<String>);
}

impl<T: Debug> ShouldSatisfyExtension for T {
    fn should_satisfy(&self, predicate: impl FnMut(&Self) -> bool, custom_message: Option<String>) {
    }
}