use crate::context::AssertionContext;

use std::fmt::Debug;

pub(crate) fn generate_message<T: Debug, O: Debug>(
    actual: T,
    expected: O,
    context: AssertionContext<T>,
) -> String {
    let mapped_actual = (context.actual_mapper)(actual);

    format!(
        "{} {} {expected:?} but was{mapped_actual}",
        context.asserted_expression, context.verb
    )
}
