use std::fmt::{self, Debug, Display, Formatter};

use crate::{ast_v2::statements::BlockStatement, environment::env::Env};

#[derive(Clone)]
pub enum Object {
    // ===== Value =====
    Null,
    Base(BaseValue),

    // ===== Statement =====
    Return(Box<Object>),
}

#[derive(PartialEq, Eq)]
pub enum ObjectType {
    Null,
    Integer,
    Float,
    Boolean,

    Return,
}

impl Object {
    pub fn obj_type(&self) -> ObjectType {
        match self {
            Object::Null => ObjectType::Null,
            Object::Base(bv) => match bv {
                BaseValue::Integer(_) => ObjectType::Integer,
                BaseValue::Float(_) => ObjectType::Float,
                BaseValue::Boolean(_) => ObjectType::Boolean,
            },
            Object::Return(_) => ObjectType::Return,
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            Object::Null => true,
            _ => false,
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Object::Null => write!(f, "null"),
            Object::Base(bv) => match bv {
                BaseValue::Integer(v) => write!(f, "{}", v.value()),
                BaseValue::Float(v) => write!(f, "{}", v.value()),
                BaseValue::Boolean(v) => write!(f, "{}", v.value()),
            },
            Object::Return(bv) => match bv.as_ref() {
                Object::Base(bv) => match bv {
                    BaseValue::Integer(v) => write!(f, "return {}", v.value()),
                    BaseValue::Float(v) => write!(f, "return {}", v.value()),
                    BaseValue::Boolean(v) => write!(f, "return {}", v.value()),
                },
                _ => write!(f, "null"),
            },
        }
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Object::Null => write!(f, "null"),
            Object::Base(bv) => match bv {
                BaseValue::Integer(v) => write!(f, "{:?}", v),
                BaseValue::Float(v) => write!(f, "{:?}", v),
                BaseValue::Boolean(v) => write!(f, "{:?}", v),
            },
            Object::Return(bv) => match bv.as_ref() {
                Object::Base(bv) => match bv {
                    BaseValue::Integer(v) => write!(f, "return {:?}", v),
                    BaseValue::Float(v) => write!(f, "return {:?}", v),
                    BaseValue::Boolean(v) => write!(f, "return {:?}", v),
                },
                _ => write!(f, "null"),
            },
        }
    }
}

pub struct Function {
    parameters: Vec<Object>,
    body: BlockStatement,
    env: Env,
}

impl Function {
    pub fn new(parameters: Vec<Object>, body: BlockStatement, env: Env) -> Self {
        Self {
            parameters,
            body,
            env,
        }
    }

    pub fn parameters(&self) -> &Vec<Object> {
        &self.parameters
    }

    pub fn body(&self) -> &BlockStatement {
        &self.body
    }

    pub fn env(&self) -> &Env {
        &self.env
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "fn({:?}) {{ {:?} }}", self.parameters, self.body)
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "fn({}) {{ {} }}",
            self.parameters
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            self.body
        )
    }
}

#[derive(Clone)]
pub enum BaseValue {
    Integer(Value<i64>),
    Float(Value<f64>),
    Boolean(Value<bool>),
}

/// This is basic value struct that holds a value of integer, float, char and boolean
/// For example, Value::new(10) will create a Value struct that holds an integer value of 10
#[derive(Clone)]
pub struct Value<T> {
    value: T,
}

impl<T> Value<T> {
    pub fn new(value: T) -> Self {
        Value { value }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn is_zero(&self) -> bool
    where
        T: PartialEq + Default,
    {
        self.value == Default::default()
    }
}

impl<T> Debug for Value<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl<T> Display for Value<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
