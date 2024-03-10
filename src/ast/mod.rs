use std::cell::RefCell;

pub mod statement;
pub mod expression;

// This module is used to define the AST node.
pub trait Node {
    // This method returns the token literal.
    fn token_literal(&self) -> String;

    // This method returns the content of the node as a string.
    fn to_string(&self) -> String;
}

// This trait is used to define the statement node.
// e.g. let a = 1; function add(a, b) { a + b; } etc.
pub trait Statement: Node {
    // This method just marks the struct as a statement node.
    fn statement_node(&self);
}

// This trait is used to define the expression node.
// e.g. 1 + 1; if (a > 1) { a + 1; } etc.
pub trait Expression: Node {
    // This method just marks the struct as an expression node.
    fn expression_node(&self);
}

// This struct stores the program information that the parser will analyze.
// In the same time, this is the root node of the AST.
pub struct Program {
    pub statements: RefCell<Vec<Box<dyn Statement>>>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: RefCell::new(Vec::new())
        }
    }

    // Add an AST node.
    pub fn push(&self, statement: Box<dyn Statement>) {
        self.statements.borrow_mut().push(statement);
    }
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.borrow().len() > 0 {
            self.statements.borrow()[0].token_literal()
        } else {
            String::new()
        }
    }

    fn to_string(&self) -> String {
        let mut out = String::new();

        for statement in self.statements.borrow().iter() {
            out.push_str(&statement.to_string());
        }

        out
    }
}