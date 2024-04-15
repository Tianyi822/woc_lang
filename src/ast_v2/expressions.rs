use std::{fmt::{Debug, Display}, str};

use super::Expression;

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

impl Debug for IdentifierExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl Display for IdentifierExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// The number expression represents an integer or float number.
pub struct NumExp {
    integer_value: Option<i64>,
    float_value: Option<f64>,
}

impl NumExp {
    pub fn new(integer_value: Option<i64>, float_value: Option<f64>) -> Self {
        Self {
            integer_value,
            float_value,
        }
    }

    pub fn integer_value(&self) -> Option<i64> {
        self.integer_value
    }

    pub fn float_value(&self) -> Option<f64> {
        self.float_value
    }
}

impl Debug for NumExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.integer_value, &self.float_value) {
            (Some(i_value), None) => write!(f, "{:?}", i_value),
            (None, Some(f_value)) => write!(f, "{:?}", f_value),
            _ => panic!("This is not a number expression."),
        }
    }
}

impl Display for NumExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.integer_value, &self.float_value) {
            (Some(i_value), None) => write!(f, "{}", i_value),
            (None, Some(f_value)) => write!(f, "{}", f_value),
            _ => panic!("This is not a number expression."),
        }
    }
}

pub struct PrefixExp {
    operator: String,
    right: Box<Expression>,
}

impl PrefixExp {
    pub fn new(operator: String, right: Expression) -> Self {
        Self { operator, right: Box::new(right)}
    }

    pub fn operator(&self) -> &str {
        &self.operator
    }

    pub fn right(&self) -> &Expression {
        &self.right
    }
}

impl Debug for PrefixExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{:?})", self.operator, self.right)
    }
}

impl Display for PrefixExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.operator, self.right)
    }
}