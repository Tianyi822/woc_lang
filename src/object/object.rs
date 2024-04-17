pub enum Object {
    Null(Null),
    Base(BaseValue),
}

pub enum BaseValue {
    Integer(Value<i64>),
    Float(Value<f64>),
    Boolean(Value<bool>),
}

/// Null is a struct that holds a null value
pub struct Null;

/// This is basic value struct that holds a value of integer, float, char and boolean
/// For example, Value::new(10) will create a Value struct that holds an integer value of 10
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
}
