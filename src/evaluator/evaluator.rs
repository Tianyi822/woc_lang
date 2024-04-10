use crate::{
    ast::ast::{Program, Statement},
    object::{base_types::Null, object::Object},
};

pub fn eval(program: &Program) -> Box<dyn Object> {
    eval_statements(&program.statements.borrow())
}

fn eval_statements(stmts: &Vec<Box<dyn Statement>>) -> Box<dyn Object> {
    let mut result: Box<dyn Object> = Box::new(Null);

    for stmt in stmts {
        result = eval_node(stmt);
    }

    result
}

fn eval_node(node: &Box<dyn Statement>) -> Box<dyn Object> {
    let _ = node;
    todo!("eval_node")
}
