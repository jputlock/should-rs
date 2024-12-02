use std::fmt::Debug;
use std::sync::{LazyLock, Mutex};

use crate::context::AssertionContext;

static ASSERTION_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

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

    register_hook_and_panic(&message);
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

    register_hook_and_panic(&message);
}

fn register_hook_and_panic(message: &str) {
    // Super basic / slow synchronization across threads
    let _guard = ASSERTION_LOCK.lock().expect("lock poisoned - bail");

    // Grab the default hook to handle "outside" panics.
    let default_hook = std::panic::take_hook();

    // Set the panic hook to this crate's hook, with the default hook as backup.
    std::panic::set_hook(Box::new(move |info| {
        crate::panic::test_hook(info, &default_hook)
    }));

    panic!("{message}");
}
