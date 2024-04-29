use crate::{
    ast_v2::{
        expressions::{ElseExp, IdentifierExp, IfExp, InfixExp, PrefixExp},
        statements::{BlockStatement, LetStatement, ReturnStatement},
        Expression, Node, Statement,
    },
    environment::env::Env,
    object::object::{BaseValue, Object, Value},
    token::token::TokenType,
};

pub fn eval(node: &Node, env: &Env) -> Object {
    match node {
        Node::Exp(exp) => eval_exp(exp, env),
        Node::Stmt(stmt) => eval_stmt(stmt, env),
    }
}

// =================== Evaluate Expression ===================

fn eval_exp(exp: &Expression, env: &Env) -> Object {
    match exp {
        Expression::Num(num) => match num.integer_value() {
            Some(value) => Object::Base(BaseValue::Integer(Value::new(value))),
            None => Object::Base(BaseValue::Float(Value::new(num.float_value().unwrap()))),
        },
        Expression::Boolean(b) => Object::Base(BaseValue::Boolean(Value::new(b.value()))),
        Expression::Prefix(pre_exp) => eval_prefix_exp(pre_exp, env),
        Expression::Infix(infix_exp) => eval_infix_exp(infix_exp, env),
        Expression::If(if_exp) => eval_if_exp(if_exp, env),
        Expression::Identifier(ident_exo) => eval_ident_exp(ident_exo, env),
        _ => Object::Null,
    }
}

fn eval_if_exp(exp: &IfExp, env: &Env) -> Object {
    let condition = eval_exp(exp.condition(), env);

    if is_truthy(&condition) {
        // if condition is true
        return eval_block_stmt(exp.consequence(), env);
    } else if exp.else_exp().is_some() {
        // if condition is false and there is an else expression
        return eval_else_exp(exp.else_exp().unwrap(), env);
    } else {
        // if condition is false and there is no else expression
        return Object::Null;
    }
}

fn eval_else_exp(exp: &ElseExp, env: &Env) -> Object {
    if exp.if_exp().is_some() {
        return eval_exp(exp.if_exp().unwrap(), env);
    } else {
        return eval_block_stmt(exp.consequence().unwrap(), env);
    }
}

fn eval_ident_exp(exp: &IdentifierExp, env: &Env) -> Object {
    match env.get(exp.value()) {
        Some(v) => v,
        None => Object::Null,
    }
}

/// Evaluate prefix expression
/// For example, !true, -5, !5, !!true, !!false, !!5
fn eval_prefix_exp(exp: &PrefixExp, env: &Env) -> Object {
    match exp.operator() {
        TokenType::Not => {
            let right = eval_exp(exp.right(), env);
            match right {
                Object::Base(BaseValue::Boolean(v)) => {
                    Object::Base(BaseValue::Boolean(Value::new(!v.value())))
                }
                Object::Base(BaseValue::Integer(v)) => {
                    Object::Base(BaseValue::Boolean(Value::new(v.is_zero())))
                }
                Object::Base(BaseValue::Float(v)) => {
                    Object::Base(BaseValue::Boolean(Value::new(v.is_zero())))
                }
                _ => Object::Null,
            }
        }
        TokenType::Minus => {
            let right = eval_exp(exp.right(), env);
            match right {
                Object::Base(BaseValue::Integer(v)) => {
                    Object::Base(BaseValue::Integer(Value::new(-v.value())))
                }
                Object::Base(BaseValue::Float(v)) => {
                    Object::Base(BaseValue::Float(Value::new(-v.value())))
                }
                _ => Object::Null,
            }
        }
        _ => Object::Null,
    }
}

/// Evaluate infix expression
/// For example:
/// - 5 + 5
/// - 5 - 5
/// - 5 * 5
/// - 5 / 5
/// - 5 == 5
/// - 5 != 5
fn eval_infix_exp(exp: &InfixExp, env: &Env) -> Object {
    let left = eval_exp(exp.left(), env);
    let right = eval_exp(exp.right(), env);
    match exp.operator() {
        TokenType::And
        | TokenType::Or
        | TokenType::EqualTo
        | TokenType::NotEqualTo
        | TokenType::Less
        | TokenType::LessThanOrEqualTo
        | TokenType::Greater
        | TokenType::GreaterThanOrEqualTo => match (left, right) {
            // Boolean compare with Boolean
            (Object::Base(BaseValue::Boolean(l)), Object::Base(BaseValue::Boolean(r))) => {
                match exp.operator() {
                    TokenType::And => {
                        Object::Base(BaseValue::Boolean(Value::new(*l.value() && *r.value())))
                    }
                    TokenType::Or => {
                        Object::Base(BaseValue::Boolean(Value::new(*l.value() || *r.value())))
                    }
                    TokenType::EqualTo => {
                        Object::Base(BaseValue::Boolean(Value::new(l.value() == r.value())))
                    }
                    TokenType::NotEqualTo => {
                        Object::Base(BaseValue::Boolean(Value::new(l.value() != r.value())))
                    }
                    _ => Object::Base(BaseValue::Boolean(Value::new(false))),
                }
            }

            // Boolean compare with Integer
            (Object::Base(BaseValue::Boolean(l)), Object::Base(BaseValue::Integer(r))) => {
                match exp.operator() {
                    TokenType::And => {
                        Object::Base(BaseValue::Boolean(Value::new(*l.value() && !r.is_zero())))
                    }
                    TokenType::Or => {
                        Object::Base(BaseValue::Boolean(Value::new(*l.value() || !r.is_zero())))
                    }
                    _ => Object::Base(BaseValue::Boolean(Value::new(false))),
                }
            }

            // Integer compare with Boolean
            (Object::Base(BaseValue::Integer(l)), Object::Base(BaseValue::Boolean(r))) => {
                match exp.operator() {
                    TokenType::And => {
                        Object::Base(BaseValue::Boolean(Value::new(!l.is_zero() && *r.value())))
                    }
                    TokenType::Or => {
                        Object::Base(BaseValue::Boolean(Value::new(!l.is_zero() || *r.value())))
                    }
                    _ => Object::Base(BaseValue::Boolean(Value::new(false))),
                }
            }

            // Boolean compare with Float
            (Object::Base(BaseValue::Boolean(l)), Object::Base(BaseValue::Float(r))) => {
                match exp.operator() {
                    TokenType::And => {
                        Object::Base(BaseValue::Boolean(Value::new(*l.value() && !r.is_zero())))
                    }
                    TokenType::Or => {
                        Object::Base(BaseValue::Boolean(Value::new(*l.value() || !r.is_zero())))
                    }
                    _ => Object::Base(BaseValue::Boolean(Value::new(false))),
                }
            }

            // Float compare with Boolean
            (Object::Base(BaseValue::Float(l)), Object::Base(BaseValue::Boolean(r))) => {
                match exp.operator() {
                    TokenType::And => {
                        Object::Base(BaseValue::Boolean(Value::new(!l.is_zero() && *r.value())))
                    }
                    TokenType::Or => {
                        Object::Base(BaseValue::Boolean(Value::new(!l.is_zero() || *r.value())))
                    }
                    _ => Object::Base(BaseValue::Boolean(Value::new(false))),
                }
            }

            // Integer compare with Integer
            (Object::Base(BaseValue::Integer(l)), Object::Base(BaseValue::Integer(r))) => {
                match exp.operator() {
                    TokenType::And => {
                        Object::Base(BaseValue::Boolean(Value::new(!l.is_zero() && !r.is_zero())))
                    }
                    TokenType::Or => {
                        Object::Base(BaseValue::Boolean(Value::new(!l.is_zero() || !r.is_zero())))
                    }
                    TokenType::EqualTo => {
                        Object::Base(BaseValue::Boolean(Value::new(l.value() == r.value())))
                    }
                    TokenType::NotEqualTo => {
                        Object::Base(BaseValue::Boolean(Value::new(l.value() != r.value())))
                    }
                    TokenType::Less => {
                        Object::Base(BaseValue::Boolean(Value::new(l.value() < r.value())))
                    }
                    TokenType::LessThanOrEqualTo => {
                        Object::Base(BaseValue::Boolean(Value::new(l.value() <= r.value())))
                    }
                    TokenType::Greater => {
                        Object::Base(BaseValue::Boolean(Value::new(l.value() > r.value())))
                    }
                    TokenType::GreaterThanOrEqualTo => {
                        Object::Base(BaseValue::Boolean(Value::new(l.value() >= r.value())))
                    }
                    _ => Object::Base(BaseValue::Boolean(Value::new(false))),
                }
            }

            // Float compare with Float
            (Object::Base(BaseValue::Float(l)), Object::Base(BaseValue::Float(r))) => {
                match exp.operator() {
                    TokenType::And => {
                        Object::Base(BaseValue::Boolean(Value::new(!l.is_zero() && !r.is_zero())))
                    }
                    TokenType::Or => {
                        Object::Base(BaseValue::Boolean(Value::new(!l.is_zero() || !r.is_zero())))
                    }
                    TokenType::EqualTo => {
                        Object::Base(BaseValue::Boolean(Value::new(l.value() == r.value())))
                    }
                    TokenType::NotEqualTo => {
                        Object::Base(BaseValue::Boolean(Value::new(l.value() != r.value())))
                    }
                    TokenType::Less => {
                        Object::Base(BaseValue::Boolean(Value::new(l.value() < r.value())))
                    }
                    TokenType::LessThanOrEqualTo => {
                        Object::Base(BaseValue::Boolean(Value::new(l.value() <= r.value())))
                    }
                    TokenType::Greater => {
                        Object::Base(BaseValue::Boolean(Value::new(l.value() > r.value())))
                    }
                    TokenType::GreaterThanOrEqualTo => {
                        Object::Base(BaseValue::Boolean(Value::new(l.value() >= r.value())))
                    }
                    _ => Object::Base(BaseValue::Boolean(Value::new(false))),
                }
            }

            // Float compare with Integer
            (Object::Base(BaseValue::Float(l)), Object::Base(BaseValue::Integer(r))) => {
                match exp.operator() {
                    TokenType::EqualTo => Object::Base(BaseValue::Boolean(Value::new(
                        *l.value() == *r.value() as f64,
                    ))),
                    TokenType::NotEqualTo => Object::Base(BaseValue::Boolean(Value::new(
                        *l.value() != *r.value() as f64,
                    ))),
                    TokenType::Less => Object::Base(BaseValue::Boolean(Value::new(
                        *l.value() < *r.value() as f64,
                    ))),
                    TokenType::LessThanOrEqualTo => Object::Base(BaseValue::Boolean(Value::new(
                        *l.value() <= *r.value() as f64,
                    ))),
                    TokenType::Greater => Object::Base(BaseValue::Boolean(Value::new(
                        *l.value() > *r.value() as f64,
                    ))),
                    TokenType::GreaterThanOrEqualTo => Object::Base(BaseValue::Boolean(
                        Value::new(*l.value() >= *r.value() as f64),
                    )),
                    _ => Object::Base(BaseValue::Boolean(Value::new(false))),
                }
            }

            // Integer compare with Float
            (Object::Base(BaseValue::Integer(l)), Object::Base(BaseValue::Float(r))) => {
                match exp.operator() {
                    TokenType::EqualTo => Object::Base(BaseValue::Boolean(Value::new(
                        *l.value() as f64 == *r.value(),
                    ))),
                    TokenType::NotEqualTo => Object::Base(BaseValue::Boolean(Value::new(
                        *l.value() as f64 != *r.value(),
                    ))),
                    TokenType::Less => Object::Base(BaseValue::Boolean(Value::new(
                        (*l.value() as f64) < *r.value(),
                    ))),
                    TokenType::LessThanOrEqualTo => Object::Base(BaseValue::Boolean(Value::new(
                        *l.value() as f64 <= *r.value(),
                    ))),
                    TokenType::Greater => Object::Base(BaseValue::Boolean(Value::new(
                        *l.value() as f64 > *r.value(),
                    ))),
                    TokenType::GreaterThanOrEqualTo => Object::Base(BaseValue::Boolean(
                        Value::new(*l.value() as f64 >= *r.value()),
                    )),
                    _ => Object::Base(BaseValue::Boolean(Value::new(false))),
                }
            }
            _ => Object::Base(BaseValue::Boolean(Value::new(false))),
        },

        TokenType::Plus | TokenType::Minus | TokenType::Asterisk | TokenType::Slash => {
            match (left, right) {
                (Object::Base(BaseValue::Integer(l)), Object::Base(BaseValue::Integer(r))) => {
                    match exp.operator() {
                        TokenType::Plus => {
                            Object::Base(BaseValue::Integer(Value::new(l.value() + r.value())))
                        }
                        TokenType::Minus => {
                            Object::Base(BaseValue::Integer(Value::new(l.value() - r.value())))
                        }
                        TokenType::Asterisk => {
                            Object::Base(BaseValue::Integer(Value::new(l.value() * r.value())))
                        }
                        TokenType::Slash => {
                            Object::Base(BaseValue::Integer(Value::new(l.value() / r.value())))
                        }
                        _ => Object::Null,
                    }
                }
                (Object::Base(BaseValue::Float(l)), Object::Base(BaseValue::Float(r))) => {
                    match exp.operator() {
                        TokenType::Plus => {
                            Object::Base(BaseValue::Float(Value::new(l.value() + r.value())))
                        }
                        TokenType::Minus => {
                            Object::Base(BaseValue::Float(Value::new(l.value() - r.value())))
                        }
                        TokenType::Asterisk => {
                            Object::Base(BaseValue::Float(Value::new(l.value() * r.value())))
                        }
                        TokenType::Slash => {
                            Object::Base(BaseValue::Float(Value::new(l.value() / r.value())))
                        }
                        _ => Object::Null,
                    }
                }
                (Object::Base(BaseValue::Integer(l)), Object::Base(BaseValue::Float(r))) => {
                    match exp.operator() {
                        TokenType::Plus => Object::Base(BaseValue::Float(Value::new(
                            *l.value() as f64 + r.value(),
                        ))),
                        TokenType::Minus => Object::Base(BaseValue::Float(Value::new(
                            *l.value() as f64 - r.value(),
                        ))),
                        TokenType::Asterisk => Object::Base(BaseValue::Float(Value::new(
                            *l.value() as f64 * r.value(),
                        ))),
                        TokenType::Slash => Object::Base(BaseValue::Float(Value::new(
                            *l.value() as f64 / r.value(),
                        ))),
                        _ => Object::Null,
                    }
                }
                (Object::Base(BaseValue::Float(l)), Object::Base(BaseValue::Integer(r))) => {
                    match exp.operator() {
                        TokenType::Plus => Object::Base(BaseValue::Float(Value::new(
                            l.value() + *r.value() as f64,
                        ))),
                        TokenType::Minus => Object::Base(BaseValue::Float(Value::new(
                            l.value() - *r.value() as f64,
                        ))),
                        TokenType::Asterisk => Object::Base(BaseValue::Float(Value::new(
                            l.value() * *r.value() as f64,
                        ))),
                        TokenType::Slash => Object::Base(BaseValue::Float(Value::new(
                            l.value() / *r.value() as f64,
                        ))),
                        _ => Object::Null,
                    }
                }
                _ => Object::Null,
            }
        }
        _ => Object::Null,
    }
}

// =================== Evaluate Statement ===================

fn eval_stmt(stmt: &Statement, env: &Env) -> Object {
    match stmt {
        Statement::Let(let_stmt) => eval_let_stmt(let_stmt, env),
        Statement::Return(ret_stmt) => eval_return_stmt(ret_stmt, env),
        Statement::Block(block_stmt) => eval_block_stmt(block_stmt, env),
        Statement::Func(_) => todo!("Implement FuncStatement evaluation"),
    }
}

fn eval_let_stmt(stmt: &LetStatement, env: &Env) -> Object {
    let value = eval_exp(stmt.value().unwrap(), env);
    env.set(stmt.name().to_string(), value);
    Object::Null
}

fn eval_block_stmt(stmt: &BlockStatement, env: &Env) -> Object {
    let mut result = Object::Null;
    for s in stmt.statements() {
        result = eval(s, env);

        match result {
            Object::Return(_) => return result,
            _ => {}
        }
    }
    result
}

fn eval_return_stmt(stmt: &ReturnStatement, env: &Env) -> Object {
    let ret_val = match stmt.value() {
        Some(v) => eval_exp(v, env),
        None => Object::Null,
    };

    match ret_val {
        Object::Base(BaseValue::Integer(v)) => Object::Return(BaseValue::Integer(v)),
        Object::Base(BaseValue::Float(v)) => Object::Return(BaseValue::Float(v)),
        Object::Base(BaseValue::Boolean(v)) => Object::Return(BaseValue::Boolean(v)),
        _ => Object::Null,
    }
}

// =================== Helper Functions ===================

fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Base(BaseValue::Boolean(v)) => *v.value(),
        Object::Base(BaseValue::Integer(v)) => !v.is_zero(),
        Object::Base(BaseValue::Float(v)) => !v.is_zero(),
        _ => false,
    }
}
