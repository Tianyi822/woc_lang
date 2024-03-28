use crate::ast::ast::{Expression, Node, Statement};
use crate::ast::expression::IdentifierExp;
use crate::token::token::Token;

// Because we need to cope with expression as statement,
// we need to create a new struct to represent it.
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<Box<dyn Expression>>,
}

impl ExpressionStatement {
    pub fn new(token: Token, expression: Option<Box<dyn Expression>>) -> ExpressionStatement {
        ExpressionStatement { token, expression }
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal().to_string()
    }

    fn to_string(&self) -> String {
        self.expression
            .as_ref()
            .map_or(String::new(), |e| e.to_string())
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
}

pub struct LetStatement {
    pub token: Token,
    pub name: IdentifierExp,
    pub value: Option<Box<dyn Expression>>,
}

// Let statement has a name, and the value is optional.
impl LetStatement {
    pub fn new(
        token: Token,
        name: IdentifierExp,
        value: Option<Box<dyn Expression>>,
    ) -> LetStatement {
        LetStatement { token, name, value }
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal().to_string()
    }

    fn to_string(&self) -> String {
        let mut out = String::new();

        out.push_str(&self.token.literal());
        out.push_str(" ");
        out.push_str(&self.name.to_string());

        if self.value.is_some() {
            out.push_str(" = ");
            out.push_str(&self.value.as_ref().map_or(String::new(), |v| v.to_string()));
        }

        out.push_str(";");

        out
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

// Return statement just store a expression.
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Box<dyn Expression>>,
}

impl ReturnStatement {
    pub fn new(token: Token, return_value: Option<Box<dyn Expression>>) -> ReturnStatement {
        ReturnStatement {
            token,
            return_value,
        }
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal().to_string()
    }

    fn to_string(&self) -> String {
        let mut out = String::new();

        out.push_str(&self.token.literal());
        out.push_str(" ");

        if self.return_value.is_some() {
            out.push_str(
                &self
                    .return_value
                    .as_ref()
                    .map_or(String::new(), |v| v.to_string()),
            );
        }

        out.push_str(";");

        out
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}

// Block statement is a list of statements.
pub struct BlockStatement {
    token: Token,
    statements: Vec<Box<dyn Statement>>,
}

impl BlockStatement {
    pub fn new(token: Token, statements: Vec<Box<dyn Statement>>) -> BlockStatement {
        BlockStatement { token, statements }
    }
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal().to_string()
    }

    fn to_string(&self) -> String {
        let mut out = String::new();

        out.push_str("{ ");
        for s in &self.statements {
            out.push_str(&s.to_string());
            out.push_str("; ")
        }
        out.push_str("}");

        out
    }
}

impl Statement for BlockStatement {
    fn statement_node(&self) {}
}
