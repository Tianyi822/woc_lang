use once_cell::sync::Lazy;

use super::object::{OBJType, Object};

/// Integer is a struct that holds an integer value
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Integer {
    value: i64,
}

impl Integer {
    pub fn new(value: i64) -> Self {
        Integer { value }
    }
}

impl Object for Integer {
    fn obj_type(&self) -> OBJType {
        OBJType::IntegerObj
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

/// Boolean is a struct that holds a boolean value
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Boolean {
    value: bool,
}

impl Boolean {
    pub fn new(value: bool) -> Self {
        Boolean { value }
    }
}

impl Object for Boolean {
    fn obj_type(&self) -> OBJType {
        OBJType::BooleanObj
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

/// Null is a struct that holds a null value
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Null;

impl Object for Null {
    fn obj_type(&self) -> OBJType {
        OBJType::NullObj
    }

    fn inspect(&self) -> String {
        "null".to_string()
    }
}

/// Ensure that this will be the only [Null] instance in the whole evaluation process.
pub static NULL: Lazy<Null> = Lazy::new(|| Null);
