use backtrace::{Backtrace, BacktraceFmt, BacktraceFrame, PrintFmt};
use std::{any::Any, panic::PanicHookInfo};

use crate::code_grabber;

pub(crate) fn test_hook(
    info: &PanicHookInfo<'_>,
    default_hook: &Box<dyn Fn(&PanicHookInfo<'_>) + Sync + Send + 'static>,
) {
    // https://github.com/rust-lang/rust/blob/4af7fa79a0e829c0edcc93434a8c788be8ec58c6/library/std/src/panicking.rs#L262-L263

    let location = info.location().expect("panics must provide a location");
    let assertion_message = payload_as_str(info.payload());

    let thread = std::thread::current();
    let thread_name = thread.name().unwrap_or("<unnamed>");

    let backtrace = Backtrace::new();

    let frame_number = backtrace.frames().iter().position(|frame| {
        frame.symbols().iter().any(|x| {
            x.name()
                .is_some_and(|name| name.to_string().contains("<T as should::extensions::"))
        })
    });

    match frame_number {
        // If 'None' was returned, an assertion via this library did not cause
        // the panic, so call the original panic hook.
        None => (default_hook)(info),

        // If 'Some' was returned, this library caused the panic. Omit the stack
        // frames that were created by this library's functions.
        Some(mut frame_num) => {
            frame_num += 1;
            let backtrace_string = format!(
                "Assertion failed:\n{:?}",
                BacktraceSubset {
                    frames: &backtrace.frames()[frame_num..]
                }
            );

            let location_string = match between(&backtrace_string, "at ", "\n") {
                Some(value) => value,
                None => &location.to_string(),
            };

            let assertion_fn = {
                let backing = format!(
                    "{:?}",
                    BacktraceSubset {
                        frames: &backtrace.frames()[frame_num - 1..frame_num]
                    }
                );
                let first_line = backing.split("\n").next().unwrap();
                code_grabber::get_assertion_function(first_line)
            };
            let code_snippet = code_grabber::get_code_snippet(location_string, &assertion_fn);

            eprintln!("Assertion failed on thread '{thread_name}' at {location_string}:\n'{code_snippet}'{assertion_message}\n\n{backtrace_string}");
        }
    };
}

// Used to create a subset of a backtrace. This is useful when omitting frames
// that are created by this library.
struct BacktraceSubset<'a> {
    frames: &'a [BacktraceFrame],
}

impl std::fmt::Debug for BacktraceSubset<'_> {
    // Heavily based on https://docs.rs/backtrace/latest/src/backtrace/capture.rs.html#487-503
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut print_path =
            move |fmt: &mut std::fmt::Formatter<'_>, path: backtrace::BytesOrWideString<'_>| {
                let path = path.into_path_buf();
                std::fmt::Display::fmt(&path.display(), fmt)
            };

        let mut f = BacktraceFmt::new(f, PrintFmt::Short, &mut print_path);

        f.add_context()?;
        for frame in self.frames {
            f.frame().backtrace_frame(frame)?;
        }
        f.finish()?;
        Ok(())
    }
}

fn payload_as_str(payload: &dyn Any) -> &str {
    if let Some(&s) = payload.downcast_ref::<&'static str>() {
        s
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.as_str()
    } else {
        "Box<dyn Any>"
    }
}

fn between<'a>(source: &'a str, start: &'a str, end: &'a str) -> Option<&'a str> {
    let start_position = source.find(start)? + start.len();
    let source = &source[start_position..];
    let end_position = source.find(end)?;
    return Some(&source[..end_position]);
}
