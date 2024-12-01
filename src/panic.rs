use backtrace;
use std::{any::Any, panic::PanicHookInfo};

fn payload_as_str(payload: &dyn Any) -> &str {
    if let Some(&s) = payload.downcast_ref::<&'static str>() {
        s
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.as_str()
    } else {
        "Box<dyn Any>"
    }
}

pub(crate) fn test_hook(info: &PanicHookInfo<'_>) {
    // https://github.com/rust-lang/rust/blob/4af7fa79a0e829c0edcc93434a8c788be8ec58c6/library/std/src/panicking.rs#L262-L263
    let location = info.location().expect("panics should provide a location");
    let assertion_message = payload_as_str(info.payload());

    let thread = std::thread::current();
    let thread_name = thread.name().unwrap_or("<unnamed>");

    let backtrace = backtrace::Backtrace::new();

    let position = backtrace.frames().iter().position(|frame| {
        frame.symbols().iter().any(|x| {
            x.name()
                .is_some_and(|name| name.to_string().contains("<T as should::extensions::"))
        })
    });

    let bt = match position {
        // If 'None' was returned, an assertion via this library did not cause
        // the panic, so yield a full backtrace.
        None => format!("{backtrace:?}"),
        Some(mut pos) => {
            pos += 1;
            format!("Assertion failed in frame {pos}:\n{backtrace:?}")
        }
    };

    // TODO: fix the 'location', which will currently always return the `assertion.rs` file.
    eprintln!(
        "Assertion failed on thread {thread_name} at {location}:\n{assertion_message}\n\n{bt}"
    );
}
