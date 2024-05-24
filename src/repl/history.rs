use std::cell::{Cell, RefCell};

pub struct History {
    history: RefCell<Vec<String>>,
    current: Cell<usize>,
}

impl History {
    pub fn new() -> Self {
        Self {
            history: RefCell::new(Vec::new()),
            current: Cell::new(0),
        }
    }

    pub fn clean(&self) {
        self.history.borrow_mut().clear();
        self.current.set(0);
    }

    pub fn add(&self, item: &str) {
        self.history.borrow_mut().push(item.to_string());
        self.current.set(self.history.borrow().len());
    }

    pub fn get_last(&self) -> Option<String> {
        // If the current index is 0, it means get the latest history or top history.
        if self.current.get() != 0 {
            self.current.set(self.current.get() - 1);
        }

        Some(self.history.borrow()[self.current.get()].clone())
    }

    pub fn get_next(&self) -> Option<String> {
        if self.current.get() != self.history.borrow().len() - 1 {
            self.current.set(self.current.get() + 1);
        }

        Some(self.history.borrow()[self.current.get()].clone())
    }
}