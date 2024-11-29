use crate::context::AssertionContext;

use std::fmt::Debug;

pub trait ErrorMessageGenerator {
    fn generate_message<T: ?Sized + Debug, O: ?Sized + Debug>(
        actual: &T,
        expected: &O,
        context: &AssertionContext,
    ) -> String;
}
