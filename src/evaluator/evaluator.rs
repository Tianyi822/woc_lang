use crate::{
    ast_v2::{Expression, Node},
    object::object::{BaseValue, Object, Value},
};

pub fn eval(node: &Node) -> Object {
    match node {
        Node::Exp(exp) => eval_expression(exp),
        Node::Stmt(_) => todo!(),
    }
}

fn eval_expression(exp: &Expression) -> Object {
    match exp {
        Expression::Num(num) => match num.integer_value() {
            Some(value) => Object::Base(BaseValue::Integer(Value::new(value))),
            None => Object::Base(BaseValue::Float(Value::new(num.float_value().unwrap()))),
        },
        Expression::Boolean(b) => Object::Base(BaseValue::Boolean(Value::new(b.value()))),
        _ => Object::Null,
    }
}
