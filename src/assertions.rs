use std::fmt::Debug;

use crate::context::AssertionContext;

pub(crate) fn assert<T: ?Sized + Debug, O: ?Sized + Debug>(
    evaluator: impl FnOnce(&T) -> bool,
    actual: &T,
    expected: &O,
    context: impl Into<AssertionContext>,
    generate_message: impl FnOnce(&T, &O, &AssertionContext) -> String,
) {
    if evaluator(actual) {
        return;
    }

    let message = generate_message(actual, expected, &context.into());
    panic!("{message}");
}
