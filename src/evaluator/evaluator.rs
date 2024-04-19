use crate::{
    ast_v2::{Expression, Node},
    object::object::{BaseValue, Object, Value},
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
        Expression::Prefix(_) => eval_prefix_exp(exp),
        Expression::Infix(_) => eval_infix_exp(exp),
        _ => Object::Null,
    }
}

/// Evaluate prefix expression
/// For example, !true, -5, !5, !!true, !!false, !!5
fn eval_prefix_exp(exp: &Expression) -> Object {
    match exp {
        Expression::Prefix(pre_exp) => match pre_exp.operator() {
            "!" => {
                let right = eval_exp(pre_exp.right());
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
            "-" => {
                let right = eval_exp(pre_exp.right());
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
        },
        _ => Object::Null,
    }
}


/// Evaluate infix expression
/// For example:
/// - 5 + 5
/// - 5 - 5
/// - 5 * 5
/// - 5 / 5
fn eval_infix_exp(exp: &Expression) -> Object {
    match exp {
        Expression::Infix(infix_exp) => {
            let left = eval_exp(infix_exp.left());
            let right = eval_exp(infix_exp.right());
            match (left, right) {
                (Object::Base(BaseValue::Integer(l)), Object::Base(BaseValue::Integer(r))) => {
                    match infix_exp.operator() {
                        "+" => Object::Base(BaseValue::Integer(Value::new(l.value() + r.value()))),
                        "-" => Object::Base(BaseValue::Integer(Value::new(l.value() - r.value()))),
                        "*" => Object::Base(BaseValue::Integer(Value::new(l.value() * r.value()))),
                        "/" => Object::Base(BaseValue::Integer(Value::new(l.value() / r.value()))),
                        _ => Object::Null,
                    }
                }
                (Object::Base(BaseValue::Float(l)), Object::Base(BaseValue::Float(r))) => {
                    match infix_exp.operator() {
                        "+" => Object::Base(BaseValue::Float(Value::new(l.value() + r.value()))),
                        "-" => Object::Base(BaseValue::Float(Value::new(l.value() - r.value()))),
                        "*" => Object::Base(BaseValue::Float(Value::new(l.value() * r.value()))),
                        "/" => Object::Base(BaseValue::Float(Value::new(l.value() / r.value()))),
                        _ => Object::Null,
                    }
                }
                (Object::Base(BaseValue::Integer(l)), Object::Base(BaseValue::Float(r))) => {
                    match infix_exp.operator() {
                        "+" => Object::Base(BaseValue::Float(Value::new(
                            *l.value() as f64 + r.value(),
                        ))),
                        "-" => Object::Base(BaseValue::Float(Value::new(
                            *l.value() as f64 - r.value(),
                        ))),
                        "*" => Object::Base(BaseValue::Float(Value::new(
                            *l.value() as f64 * r.value(),
                        ))),
                        "/" => Object::Base(BaseValue::Float(Value::new(
                            *l.value() as f64 / r.value(),
                        ))),
                        _ => Object::Null,
                    }
                }
                (Object::Base(BaseValue::Float(l)), Object::Base(BaseValue::Integer(r))) => {
                    match infix_exp.operator() {
                        "+" => Object::Base(BaseValue::Float(Value::new(
                            l.value() + *r.value() as f64,
                        ))),
                        "-" => Object::Base(BaseValue::Float(Value::new(
                            l.value() - *r.value() as f64,
                        ))),
                        "*" => Object::Base(BaseValue::Float(Value::new(
                            l.value() * *r.value() as f64,
                        ))),
                        "/" => Object::Base(BaseValue::Float(Value::new(
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
