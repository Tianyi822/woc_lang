use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone)]
pub enum Object {
    Null,
    Base(BaseValue),
    Return(BaseValue),
}

#[derive(PartialEq, Eq)]
pub enum ObjectType {
    Null,
    Integer,
    Float,
    Boolean,
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
            Object::Return(bv) => match bv {
                BaseValue::Integer(_) => ObjectType::Integer,
                BaseValue::Float(_) => ObjectType::Float,
                BaseValue::Boolean(_) => ObjectType::Boolean,
            },
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
            Object::Return(bv) => match bv {
                BaseValue::Integer(v) => write!(f, "return {}", v.value()),
                BaseValue::Float(v) => write!(f, "return {}", v.value()),
                BaseValue::Boolean(v) => write!(f, "return {}", v.value()),
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
            Object::Return(bv) => match bv {
                BaseValue::Integer(v) => write!(f, "return {:?}", v),
                BaseValue::Float(v) => write!(f, "return {:?}", v),
                BaseValue::Boolean(v) => write!(f, "return {:?}", v),
            },
        }
    }
}

#[derive(Clone)]
pub enum BaseValue {
    Integer(Value<i64>),
    Float(Value<f64>),
    Boolean(Value<bool>),
}

/// Null is a struct that holds a null value
pub struct Null;

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
