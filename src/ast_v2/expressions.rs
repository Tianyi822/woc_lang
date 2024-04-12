/// The identifier expression represents a variable or function name.
/// It distinguishes itself from the implementation in the previous version by removing the token field,
/// reduce memory usage, and simplify the implementation.
pub struct IdentifierExp {
    value: String,
}

impl IdentifierExp {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
