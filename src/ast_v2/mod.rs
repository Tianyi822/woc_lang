use std::fmt::{Debug, Display};

pub mod expressions;
pub mod statements;

/// This is the AST (Abstract Syntax Tree) version 2. It is a simplified version compared to AST version 1.
/// The AST version 1 uses dynamic dispatch to handle different types of nodes, which leads to increased complexity and slower performance due to runtime type checking.
/// In contrast, the new AST version 2 utilizes enums to represent different types of nodes, eliminating the need for frequent runtime type checks and thus enhancing efficiency.
/// The new AST version 2 also provides a more straightforward and concise implementation, making it easier to understand and maintain.
/// The statement and expression nodes are defined by [`Expression`] enum and [`Statement`] enum, respectively.
#[derive(Clone)]
pub enum Node {
    Stmt(Statement),
    Exp(Expression),
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Stmt(stmt) => write!(f, "{:?}", stmt),
            Node::Exp(exp) => write!(f, "{:?}", exp),
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Stmt(stmt) => write!(f, "{}", stmt),
            Node::Exp(exp) => write!(f, "{}", exp),
        }
    }
}

/// All expression AST node types are defined in the expressions module.
/// For example:
/// - IdentifierExp: x, y, z
/// - NumExp: 1, 2, 3, 8.22, -8.22
/// - PrefixExp: -x, !y, -z
/// - InfixExp:
///     - Basic calculate operators: x + y, x - y, x * y, x / y
///     - Logical operators: x == y, x != y, x < y, x > y, x <= y, x >= y, x && y, x || y
///     The above infix expressions will be parsed into the InfixExp node.
/// - IfExp and ElseExp: if (x < y) { return x; } else { return y; }
/// - FuncExp: fn add(x, y) { return x + y; }
#[derive(Clone)]
pub enum Expression {
    Identifier(expressions::IdentifierExp),
    Num(expressions::NumExp),
    Boolean(expressions::BooleanExp),
    String(expressions::StringExp),
    If(expressions::IfExp),
    Call(expressions::CallExp),
    Prefix(expressions::PrefixExp),
    Infix(expressions::InfixExp),
}

impl Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(ident) => write!(f, "{:?}", ident),
            Expression::Num(num) => write!(f, "{:?}", num),
            Expression::Boolean(bool_exp) => write!(f, "{:?}", bool_exp),
            Expression::String(str_exp) => write!(f, "{:?}", str_exp),
            Expression::If(if_exp) => write!(f, "{:?}", if_exp),
            Expression::Call(call_exp) => write!(f, "{:?}", call_exp),
            Expression::Prefix(pre_exp) => write!(f, "{:?}", pre_exp),
            Expression::Infix(infix_exp) => write!(f, "{:?}", infix_exp),
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(ident) => write!(f, "{}", ident),
            Expression::Num(num) => write!(f, "{}", num),
            Expression::Boolean(bool_exp) => write!(f, "{}", bool_exp),
            Expression::String(str_exp) => write!(f, "{}", str_exp),
            Expression::If(if_exp) => write!(f, "{}", if_exp),
            Expression::Call(call_exp) => write!(f, "{}", call_exp),
            Expression::Prefix(pre_exp) => write!(f, "{}", pre_exp),
            Expression::Infix(infix_exp) => write!(f, "{}", infix_exp),
        }
    }
}

/// All statement AST node types are defined in the statements module.
/// For example:
/// - LetStatement: let x = 822;
/// - ReturnStatement: return x;
#[derive(Clone)]
pub enum Statement {
    Let(statements::LetStatement),
    Return(statements::ReturnStatement),
    Block(statements::BlockStatement),
    Func(statements::FuncStatement),
}

impl Debug for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let(let_stmt) => write!(f, "{:?}", let_stmt),
            Statement::Return(return_stmt) => write!(f, "{:?}", return_stmt),
            Statement::Block(block_stmt) => write!(f, "{:?}", block_stmt),
            Statement::Func(func_stmt) => write!(f, "{:?}", func_stmt),
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let(let_stmt) => write!(f, "{}", let_stmt),
            Statement::Return(return_stmt) => write!(f, "{}", return_stmt),
            Statement::Block(block_stmt) => write!(f, "{}", block_stmt),
            Statement::Func(func_stmt) => write!(f, "{}", func_stmt),
        }
    }
}
