use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::usize::MAX;

use regex::Regex;

struct CodeLocator<'a> {
    fqp: &'a Path,
    line_number: usize,
    column: usize,
}

impl<'a> CodeLocator<'a> {
    pub fn new(message: &'a str) -> Self {
        let matcher = Regex::new(r"(?<fqp>.*?):(?<line>\d+):(?<column>\d+)").unwrap();
        let captures = matcher.captures(message).unwrap();

        Self {
            fqp: Path::new(captures.name("fqp").unwrap().as_str()),
            line_number: captures.name("line").unwrap().as_str().parse().unwrap(),
            column: captures.name("column").unwrap().as_str().parse().unwrap(),
        }
    }
}

pub fn get_assertion_function<'a>(message: &str) -> String {
    let index = message.rfind("::").unwrap();
    message[index + 2..].to_string()
}

pub fn get_code_snippet(message: &str, assertion_fn: &str) -> String {
    let locator = CodeLocator::new(message);

    match inner(locator, assertion_fn) {
        Ok(s) => s,
        Err(e) => format!("(could not get source: {e:?})"),
    }
}

fn inner(locator: CodeLocator, assertion_fn: &str) -> Result<String, String> {
    let handle = File::open(locator.fqp).map_err(|x| x.to_string())?;
    let reader = BufReader::new(handle);

    let line = reader.lines().skip(locator.line_number - 1).next();

    match line {
        Some(line) => {
            let line = line.map_err(|x| x.to_string())?;
            if line.len() <= locator.column {
                return Err(format!("line does not have a column {}", locator.column));
            }
            let pos = line.rfind(assertion_fn).ok_or(format!(
                "line does not have assertion function: '{assertion_fn}'"
            ))?;
            Ok(line[locator.column - 1..pos - 1].to_string())
        }
        None => Err(format!("could not access line {}", locator.line_number)),
    }
}
