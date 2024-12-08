use crate::context::AssertionContext;

use std::fmt::Debug;

pub(crate) fn expected_vs_actual_message<T: Debug, O: Debug>(
    actual: T,
    expected: O,
    context: AssertionContext<T>,
) -> String {
    let mapped_actual = (context.actual_mapper)(actual);

    let custom_message = if let Some(msg) = context.custom_message {
        format!("\n\nExtra details: {msg}")
    } else {
        "".to_string()
    };

    format!(
        " {} {expected:?} but was{mapped_actual}{custom_message}",
        context.verb
    )
}

pub(crate) fn failed_condition_message<T: Debug>(
    actual: T,
    context: AssertionContext<T>,
) -> String {
    let mapped_actual = (context.actual_mapper)(actual);

    let custom_message = if let Some(msg) = context.custom_message {
        format!("\n\nExtra details: {msg}")
    } else {
        "".to_string()
    };

    format!(" {} but was{mapped_actual}{custom_message}", context.verb)
}
