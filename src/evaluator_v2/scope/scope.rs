use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::object::object::Object;

/// Scope is a struct that stores the variables and functions in the scope and the parent scope.
/// The parent scope is used to find some variables or functions that are not in the current scope until the global scope.
#[derive(Clone)]
pub struct Scope {
    // Store the variables and functions in the scope
    store: RefCell<HashMap<String, Rc<Object>>>,
    // Store the parent scope
    parent_scope: Option<Box<Scope>>,
}

impl Scope {
    /// Create a new scope with the parent scope
    pub fn new(parent_scope: Option<Box<Scope>>) -> Self {
        Self {
            store: RefCell::new(HashMap::new()),
            parent_scope,
        }
    }

    /// Get the value of the variable or function in the scope
    pub fn get(&self, name: &str) -> Option<Rc<Object>> {
        match self.store.borrow().get(name) {
            Some(object) => Some(Rc::clone(object)),
            None => match &self.parent_scope {
                Some(parent_scope) => parent_scope.get(name),
                None => None,
            },
        }
    }

    /// Set the value of the variable or function in the scope
    pub fn set(&self, name: String, object: Object) {
        self.store
            .borrow_mut()
            .insert(name, Rc::new(object));
    }
}
