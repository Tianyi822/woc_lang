use super::{expressions::IdentifierExp, Expression};

/// Let statement is a statement that binds a value to a name.
/// For example: let x = 822;
pub struct LetStatement {
    ident: IdentifierExp,
    value: Option<Expression>,
}

impl LetStatement {
    pub fn new(ident: IdentifierExp, value: Option<Expression>) -> Self {
        Self { ident, value }
    }

    /// Get the name of the let statement.
    /// For example: let x = 822; -> x
    pub fn name(&self) -> &str {
        self.ident.value()
    }

    /// Get the value of the let statement.
    /// For example: let x = 822; -> 822
    pub fn value(&self) -> Option<&Expression> {
        self.value.as_ref()
    }
}
