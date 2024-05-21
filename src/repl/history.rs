use std::cell::{Cell, RefCell};

pub struct History {
    history: RefCell<Vec<String>>,
    current: Cell<usize>,
}

impl History {
    fn new() -> Self {
        Self {
            history: RefCell::new(Vec::new()),
            current: Cell::new(0),
        }
    }

    pub(super) fn get_current(&self) -> usize {
        self.current.get()
    }

    pub(super) fn add(&self, item: String) {
        self.history.borrow_mut().push(item);
        self.current.set(self.current.get())
    }

    pub(super) fn get(&self, index: usize) -> Option<String> {
        if index < self.history.borrow().len() {
            self.current.set(index);
            Some(self.history.borrow()[index].clone())
        } else {
            None
        }
    }

    pub(super) fn get_latest_item(&self) -> Option<String> {
        if self.history.borrow().is_empty() {
            None
        } else {
            self.current.set(self.history.borrow().len() - 1);
            Some(self.history.borrow()[self.current.get()].clone())
        }
    }
}
