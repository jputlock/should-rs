use std::fmt::Debug;

use crate::context::AssertionContext;

pub(crate) fn assert_unary<'a, T: Debug>(
    actual: T,
    evaluator: impl FnOnce(T) -> bool,
    original_actual: T,
    expected_status: &'a str,
    context: impl Into<AssertionContext<T>>,
    generate_message: impl FnOnce(T, &'a str, AssertionContext<T>) -> String,
) {
    if evaluator(actual) {
        return;
    }

    let message = generate_message(original_actual, expected_status, context.into());
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
    panic!("{message}");
}
