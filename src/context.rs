pub struct AssertionContext {
    pub asserted_expression: String,
    pub verb: String,
    pub is_negated: bool,
    // pub tolerance: Option<f64>,
}

impl Default for AssertionContext {
    fn default() -> Self {
        Self {
            // TODO: should evaluate the backtrace by default
            asserted_expression: "Placeholder code".to_string(),
            verb: "should be".to_string(),
            is_negated: false,
            // tolerance: None,
        }
    }
}

impl From<AssertionContextBuilder> for AssertionContext {
    fn from(value: AssertionContextBuilder) -> Self {
        value.build()
    }
}

pub struct AssertionContextBuilder {
    context: AssertionContext,
}

impl AssertionContextBuilder {
    pub fn new() -> Self {
        AssertionContextBuilder {
            context: AssertionContext::default(),
        }
    }

    pub fn verb(mut self, value: String) -> Self {
        self.context.verb = value;
        self
    }

    pub fn is_negated(mut self, value: bool) -> Self {
        self.context.is_negated = value;
        self
    }

    // pub fn tolerance(mut self, value: Option<f64>) -> Self {
    //     self.context.tolerance = value;
    //     self
    // }

    pub(crate) fn build(self) -> AssertionContext {
        self.context
    }
}
