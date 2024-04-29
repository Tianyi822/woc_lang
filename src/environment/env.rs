use std::{
    cell::RefCell,
    collections::HashMap,
};

use crate::object::object::Object;

pub struct Env {
    store: RefCell<HashMap<String, Object>>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            store: RefCell::new(HashMap::new()),
        }
    }

    pub fn get(&self, key: &str) -> Option<Object> {
        match self.store.borrow().get(key) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    pub fn set(&self, key: String, value: Object) {
        self.store.borrow_mut().insert(key, value);
    }
}
