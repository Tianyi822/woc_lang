use std::{
    fmt::{Debug, Display},
    str,
};

use crate::token::token::TokenType;

use super::{statements::BlockStatement, Expression};

/// The identifier expression represents a variable or function name.
/// It distinguishes itself from the implementation in the previous version by removing the token field,
/// reduce memory usage, and simplify the implementation.
#[derive(Clone)]
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
#[derive(Clone)]
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

/// The boolean expression represents a boolean value.
#[derive(Clone)]
pub struct BooleanExp {
    value: bool,
}

impl BooleanExp {
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    pub fn value(&self) -> bool {
        self.value
    }
}

impl Debug for BooleanExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl Display for BooleanExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// The if expression represents the if condition, consequence, and alternative.
/// For example:
///
/// ```
/// if (x < y) {
///     return x;
/// } else {
///     return y;
/// }
/// ```
#[derive(Clone)]
pub struct IfExp {
    condition: Box<Expression>,
    consequence: BlockStatement,
    else_exp: Option<ElseExp>,
}

impl IfExp {
    pub fn new(
        condition: Expression,
        consequence: BlockStatement,
        else_exp: Option<ElseExp>,
    ) -> Self {
        Self {
            condition: Box::new(condition),
            consequence,
            else_exp,
        }
    }

    pub fn condition(&self) -> &Expression {
        &self.condition
    }

    pub fn consequence(&self) -> &BlockStatement {
        &self.consequence
    }

    pub fn else_exp(&self) -> Option<&ElseExp> {
        self.else_exp.as_ref()
    }
}

impl Debug for IfExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.else_exp.is_some() {
            write!(
                f,
                "if {:?} {:?} {:?}",
                self.condition, self.consequence, self.else_exp
            )
        } else {
            write!(f, "if {:?} {:?}", self.condition, self.consequence)
        }
    }
}

impl Display for IfExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.else_exp.is_some() {
            write!(
                f,
                "if {} {} {}",
                self.condition,
                self.consequence,
                self.else_exp.as_ref().unwrap()
            )
        } else {
            write!(f, "if {} {}", self.condition, self.consequence)
        }
    }
}

/// The else expression represents the if expression and the consequence.
/// For example:
///
/// ```
/// if (x < y) {
///     return x;
/// } else {
///     return y;
/// }
/// ```
#[derive(Clone)]
pub struct ElseExp {
    if_exp: Option<Box<IfExp>>,
    consequence: Option<BlockStatement>,
}

impl ElseExp {
    pub fn new(if_exp: Option<Box<IfExp>>, consequence: Option<BlockStatement>) -> Self {
        Self {
            if_exp,
            consequence,
        }
    }

    pub fn if_exp(&self) -> Option<&IfExp> {
        self.if_exp.as_ref().map(|exp| &**exp)
    }

    pub fn consequence(&self) -> Option<&BlockStatement> {
        self.consequence.as_ref()
    }
}

impl Debug for ElseExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.if_exp.is_some() {
            write!(f, "else {:?}", self.if_exp.as_ref().unwrap(),)
        } else {
            write!(f, "else {:?}", self.consequence.as_ref().unwrap())
        }
    }
}

impl Display for ElseExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.if_exp.is_some() {
            write!(f, "else {}", self.if_exp.as_ref().unwrap(),)
        } else {
            write!(f, "else {}", self.consequence.as_ref().unwrap())
        }
    }
}

/// The call expression represents the function and the arguments.
///
/// For example:
///
/// ```
/// add(1, 2)
/// ```
#[derive(Clone)]
pub struct CallExp {
    name: IdentifierExp,
    arguments: Vec<Expression>,
}

impl CallExp {
    pub fn new(name: IdentifierExp, arguments: Vec<Expression>) -> Self {
        Self { name, arguments }
    }

    pub fn name(&self) -> &IdentifierExp {
        &self.name
    }

    pub fn arguments(&self) -> &Vec<Expression> {
        &self.arguments
    }
}

impl Debug for CallExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({:?})", self.name, self.arguments)
    }
}

impl Display for CallExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({})",
            self.name,
            self.arguments
                .iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

/// The prefix expression represents the prefix operator and the right expression.
/// For example, !x, -y, !true, -false
#[derive(Clone)]
pub struct PrefixExp {
    operator: TokenType,
    right: Box<Expression>,
}

impl PrefixExp {
    pub fn new(operator: TokenType, right: Expression) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }

    pub fn operator(&self) -> &TokenType {
        &self.operator
    }

    pub fn right(&self) -> &Expression {
        &self.right
    }
}

impl Debug for PrefixExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:?}", self.operator, self.right)
    }
}

impl Display for PrefixExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.operator, self.right)
    }
}

/// The infix expression represents the left expression, operator, and right expression.
#[derive(Clone)]
pub struct InfixExp {
    left: Box<Expression>,
    operator: TokenType,
    right: Box<Expression>,
}

impl InfixExp {
    pub fn new(left: Expression, operator: TokenType, right: Expression) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn left(&self) -> &Expression {
        &self.left
    }

    pub fn operator(&self) -> &TokenType {
        &self.operator
    }

    pub fn right(&self) -> &Expression {
        &self.right
    }
}

impl Debug for InfixExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?} {:?} {:?})", self.left, self.operator, self.right)
    }
}

impl Display for InfixExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}
