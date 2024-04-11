use std::any::Any;

use crate::object::object::Object;

pub fn eval(program: Box<dyn Any>) -> Box<dyn Object> {
    let _ = program;
    todo!("eval")
}
