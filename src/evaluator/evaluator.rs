use crate::{
    ast_v2::{
        expressions::{IfExp, InfixExp, PrefixExp},
        Expression, Node,
    },
    object::object::{BaseValue, Object, Value},
    token::token::TokenType,
};

pub fn eval(node: &Node) -> Object {
    match node {
        Node::Exp(exp) => eval_exp(exp),
        Node::Stmt(_) => todo!(),
    }
}

fn eval_exp(exp: &Expression) -> Object {
    match exp {
        Expression::Num(num) => match num.integer_value() {
            Some(value) => Object::Base(BaseValue::Integer(Value::new(value))),
            None => Object::Base(BaseValue::Float(Value::new(num.float_value().unwrap()))),
        },
        Expression::Boolean(b) => Object::Base(BaseValue::Boolean(Value::new(b.value()))),
        Expression::Prefix(pre_exp) => eval_prefix_exp(pre_exp),
        Expression::Infix(infix_exp) => eval_infix_exp(infix_exp),
        Expression::If(if_exp) => eval_if_exp(if_exp),
        _ => Object::Null,
    }
}

/// Evaluate prefix expression
/// For example, !true, -5, !5, !!true, !!false, !!5
fn eval_prefix_exp(exp: &PrefixExp) -> Object {
    match exp.operator() {
        TokenType::Not => {
            let right = eval_exp(exp.right());
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
            let right = eval_exp(exp.right());
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

fn eval_if_exp(exp: &IfExp) -> Object {
    let _condition = eval_exp(exp.condition());
    todo!("Implement eval_if_exp")
    // match condition {
    //     Object::Base(BaseValue::Boolean(v)) => {
    //         if *v.value() {
    //             eval(exp.consequence())
    //         } else {
    //             match exp.alternative() {
    //                 Some(alt) => eval(alt),
    //                 None => Object::Null,
    //             }
    //         }
    //     }
    //     _ => Object::Null,
    // }
}

/// Evaluate infix expression
/// For example:
/// - 5 + 5
/// - 5 - 5
/// - 5 * 5
/// - 5 / 5
/// - 5 == 5
/// - 5 != 5
fn eval_infix_exp(exp: &InfixExp) -> Object {
    let left = eval_exp(exp.left());
    let right = eval_exp(exp.right());
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
