use std::fmt::{Debug, Display};

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

impl Debug for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Some(value) => {
                write!(f, "let {} = {};", self.ident, value)
            }
            None => {
                write!(f, "let {};", self.ident)
            }
        }
    }
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Some(value) => {
                write!(f, "let {} = {};", self.ident, value)
            }
            None => {
                write!(f, "let {};", self.ident)
            }
        }
    }
}

pub struct ReturnStatement {
    value: Option<Expression>,
}

impl ReturnStatement {
    pub fn new(value: Option<Expression>) -> Self {
        Self { value }
    }

    /// Get the value of the return statement.
    /// For example: return 822; -> 822
    pub fn value(&self) -> Option<&Expression> {
        self.value.as_ref()
    }
}

impl Debug for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Some(value) => {
                write!(f, "return {};", value)
            }
            None => {
                write!(f, "return;")
            }
        }
    }
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Some(value) => {
                write!(f, "return {};", value)
            }
            None => {
                write!(f, "return;")
            }
        }
    }
}
