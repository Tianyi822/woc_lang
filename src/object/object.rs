use std::fmt::{self, Debug, Display, Formatter};

use crate::{
    ast_v2::{expressions::IdentifierExp, statements::BlockStatement},
    evaluator_v2::{evaluator::Evaluator, scope::scope::Scope},
};

#[derive(Clone)]
pub enum Object {
    // ===== Value =====
    Null,
    Base(BaseValue),

    // ===== String =====
    Str(Str),

    // ===== Array =====
    Array(Array),

    // ===== Statement =====
    Return(Box<Object>),

    Func(Function),
}

#[derive(PartialEq, Eq)]
pub enum ObjectType {
    Null,
    Integer,
    Float,
    Boolean,

    String,
    Array,

    Return,
    Func,
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
            Object::Str(_) => ObjectType::String,
            Object::Array(_) => ObjectType::Array,
            Object::Return(_) => ObjectType::Return,
            Object::Func(_) => ObjectType::Func,
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
            Object::Str(str) => write!(f, "{}", str.value()),
            Object::Array(arr) => write!(f, "{:?}", arr.elements()),
            Object::Return(bv) => match bv.as_ref() {
                Object::Base(bv) => match bv {
                    BaseValue::Integer(v) => write!(f, "return {}", v.value()),
                    BaseValue::Float(v) => write!(f, "return {}", v.value()),
                    BaseValue::Boolean(v) => write!(f, "return {}", v.value()),
                },
                _ => write!(f, "null"),
            },
            Object::Func(func) => write!(f, "{}", func),
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
            Object::Str(str) => write!(f, "{:?}", str),
            Object::Array(arr) => write!(f, "{:?}", arr),
            Object::Return(bv) => match bv.as_ref() {
                Object::Base(bv) => match bv {
                    BaseValue::Integer(v) => write!(f, "return {:?}", v),
                    BaseValue::Float(v) => write!(f, "return {:?}", v),
                    BaseValue::Boolean(v) => write!(f, "return {:?}", v),
                },
                _ => write!(f, "null"),
            },
            Object::Func(func) => write!(f, "{:?}", func),
        }
    }
}

#[derive(Clone)]
pub struct Str {
    value: String,
}

impl Str {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Debug for Str {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl Display for Str {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Clone)]
pub struct Array {
   elements: Vec<Object>,
}

impl Array {
    pub fn new(elements: Vec<Object>) -> Self {
        Self { elements }
    }

    pub fn elements(&self) -> &Vec<Object> {
        &self.elements
    }

    pub fn get(&self, index: usize) -> Option<&Object> {
        self.elements.get(index)
    }
}

impl Debug for Array {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.elements)
    }
}

impl Display for Array {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.elements)
    }
}

#[derive(Clone)]
pub struct Function {
    name: String,
    parameters: Option<Vec<IdentifierExp>>,
    body: BlockStatement,
    eval: Evaluator,
}

impl Function {
    pub fn new(
        name: String,
        parameters: Option<Vec<IdentifierExp>>,
        body: BlockStatement,
        parent_scope: Option<Box<Scope>>,
    ) -> Self {
        Self {
            name,
            parameters,
            body,
            eval: Evaluator::new(parent_scope),
        }
    }

    pub fn add_arguments(&self, args: Vec<Object>) -> Result<(), String> {
        if args.len() != self.parameters.as_ref().unwrap().len() {
            return Err(format!(
                "wrong number of arguments. got={}, want={}",
                args.len(),
                self.parameters.as_ref().unwrap().len()
            ));
        }

        for (i, param) in self.parameters.as_ref().unwrap().iter().enumerate() {
            self.eval
                .scope()
                .set(param.value().to_string(), args[i].clone());
        }

        self.add_self_to_scope();

        Ok(())
    }

    fn add_self_to_scope(&self) {
        self.eval
            .scope()
            .set(self.name.clone(), Object::Func(self.clone()));
    }

    pub fn eval(&self) -> Object {
        self.eval.eval_block_stmt(&self.body)
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "fn({}) {{ {} }}",
            self.parameters
                .as_ref()
                .map(|params| {
                    params
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                })
                .unwrap_or_default(),
            self.body
        )
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "fn({}) {{ {} }}",
            self.parameters
                .as_ref()
                .map(|params| {
                    params
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                })
                .unwrap_or_default(),
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
