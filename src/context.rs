use std::fmt::Debug;

pub struct AssertionContext<T: Debug> {
    pub asserted_expression: String,
    pub verb: String,
    pub actual_mapper: Box<dyn FnOnce(T) -> String>,
    pub custom_message: Option<String>,
    // pub tolerance: Option<f64>,
}

impl<T: Debug> Default for AssertionContext<T> {
    fn default() -> Self {
        Self {
            // TODO: should evaluate the backtrace by default
            asserted_expression: "Placeholder code".to_string(),
            verb: "should be".to_string(),
            actual_mapper: Box::new(|x| format!(" {x:?}")),
            // tolerance: None,
            custom_message: None,
        }
    }
}

impl<T: Debug> From<AssertionContextBuilder<T>> for AssertionContext<T> {
    fn from(value: AssertionContextBuilder<T>) -> Self {
        value.build()
    }
}

pub struct AssertionContextBuilder<T: Debug> {
    context: AssertionContext<T>,
}

impl<T: Debug> AssertionContextBuilder<T> {
    pub fn new() -> Self {
        AssertionContextBuilder {
            context: AssertionContext::default(),
        }
    }

    pub fn verb(mut self, value: &str) -> Self {
        self.context.verb = value.to_string();
        self
    }

    pub fn actual_mapper(mut self, value: Box<dyn FnOnce(T) -> String>) -> Self {
        self.context.actual_mapper = value;
        self
    }

    // pub fn tolerance(mut self, value: Option<f64>) -> Self {
    //     self.context.tolerance = value;
    //     self
    // }

    pub fn custom_message(mut self, custom_message: Option<String>) -> Self {
        self.context.custom_message = custom_message;
        self
    }

    pub(crate) fn build(self) -> AssertionContext<T> {
        self.context
    }
}
