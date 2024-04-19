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
        _ => Object::Null,
    }
}

/// Evaluate prefix expression
/// For example, !true, -5, !5, !!true, !!false, !!5
fn eval_prefix_exp(exp: &Expression) -> Object {
    match exp {
        Expression::Prefix(pre_exp) => {
            match pre_exp.operator() {
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
            }
        }
        _ => Object::Null,
    }
}
