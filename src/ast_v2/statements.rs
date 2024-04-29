use std::fmt::{Debug, Display};

use super::{Expression, expressions::IdentifierExp, Node};

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

/// Return statement is a statement that returns a value from a function.
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

/// Block statement is a statement that groups multiple statements together.
pub struct BlockStatement {
    statements: Vec<Box<Node>>,
}

impl BlockStatement {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    /// Get the statements in the block statement.
    pub fn statements(&self) -> &Vec<Box<Node>> {
        &self.statements
    }

    /// Add a statement to the block statement.
    pub fn add(&mut self, stmt: Box<Node>) {
        self.statements.push(stmt);
    }
}

impl Debug for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stmts = self.statements.iter().map(|stmt| format!("{:?}", stmt)).collect::<Vec<String>>().join(" ");

        write!(f, "{{{}}}", stmts)
    }
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stmts = self.statements.iter().map(|stmt| format!("{:?}", stmt)).collect::<Vec<String>>().join(" ");

        write!(f, "{{{}}}", stmts)
    }
}

/// Function statement is a statement that defines a function.
/// For example: fn add(x, y) { return x + y; }
pub struct FuncStatement {
    ident: IdentifierExp,
    params: Option<Vec<IdentifierExp>>,
    body: BlockStatement,
}

impl FuncStatement {
    pub fn new(ident: IdentifierExp, params: Option<Vec<IdentifierExp>>, body: BlockStatement) -> Self {
        Self { ident, params, body }
    }

    /// Get the name of the function statement.
    /// For example: fn add(x, y) { return x + y; } -> add
    pub fn name(&self) -> &str {
        self.ident.value()
    }

    /// Get the parameters of the function statement.
    /// For example: fn add(x, y) { return x + y; } -> x, y
    pub fn params(&self) -> Option<&Vec<IdentifierExp>> {
        self.params.as_ref()
    }

    /// Get the body of the function statement.
    /// For example: fn add(x, y) { return x + y; } -> { return x + y; }
    pub fn body(self) -> BlockStatement {
        self.body
    }
}

impl Debug for FuncStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.params {
            Some(params) => {
                let params_str = params.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", ");
                write!(f, "func {}({}) {:?}", self.ident, params_str, self.body)
            }
            None => {
                write!(f, "func {}() {:?}", self.ident, self.body)
            }
        }
    }
}

impl Display for FuncStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.params {
            Some(params) => {
                let params_str = params.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", ");
                write!(f, "func {}({}) {:?}", self.ident, params_str, self.body)
            }
            None => {
                write!(f, "func {}() {:?}", self.ident, self.body)
            }
        }
    }
}
