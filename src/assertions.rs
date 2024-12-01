use std::fmt::Debug;

use crate::context::AssertionContext;

pub(crate) fn assert_unary<T: Debug>(
    actual: T,
    evaluator: impl FnOnce(T) -> bool,
    original_actual: T,
    context: impl Into<AssertionContext<T>>,
    generate_message: impl FnOnce(T, AssertionContext<T>) -> String,
) {
    if evaluator(actual) {
        return;
    }

    let message = generate_message(original_actual, context.into());
    std::panic::set_hook(Box::new(crate::panic::test_hook));
    panic!("{message}");
}

pub(crate) fn assert_comparison<T: Debug, O: Debug>(
    actual: T,
    evaluator: impl FnOnce(T) -> bool,
    original_actual: T,
    expected: O,
    context: impl Into<AssertionContext<T>>,
    generate_message: impl FnOnce(T, O, AssertionContext<T>) -> String,
) {
    if evaluator(actual) {
        return;
    }

    let message = generate_message(original_actual, expected, context.into());
    std::panic::set_hook(Box::new(crate::panic::test_hook));
    panic!("{message}");
}
