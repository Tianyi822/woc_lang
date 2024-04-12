use super::{expressions::IdentifierExp, Expression};

/// Let statement is a statement that binds a value to a name.
/// For example: let x = 822;
pub struct LetStatement {
    name: IdentifierExp,
    value: Expression,
}

impl LetStatement {
    pub fn new(name: IdentifierExp, value: Expression) -> Self {
        Self { name, value }
    }

    /// Get the name of the let statement.
    /// For example: let x = 822; -> x
    pub fn name(&self) -> &str {
        self.name.value()
    }

    /// Get the value of the let statement.
    /// For example: let x = 822; -> 822
    pub fn value(&self) -> &Expression {
        &self.value
    }
}
