use std::cell::{ Cell, RefCell };

#[derive(Debug)]
pub struct ThreadPool {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl ThreadPool {
    pub fn new() -> Self {
        ThreadPool {
            drops: Cell::new(0),
            states: RefCell::new(vec![]),
        }
    }

    pub fn new_thread(&self, c: String) -> (usize, Thread) {
        let id = self.states.borrow().len();
        self.states.borrow_mut().push(false);
        (
            id,
            Thread {
                pid: id,
                cmd: c.clone(),
                parent: self,
            },
        )
    }

    pub fn thread_len(&self) -> usize {
        self.states.borrow().len()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        if id >= self.states.borrow().len() {
            return false;
        }
        self.states.borrow()[id]
    }

    pub fn drop_thread(&self, id: usize) {
        if id < self.states.borrow().len() {
            self.states.borrow_mut()[id] = true;
            self.drops.set(self.drops.get() + 1);
        }
    }
}

#[derive(Debug)]
pub struct Thread<'a> {
    pub pid: usize,
    pub cmd: String,
    pub parent: &'a ThreadPool,
}

impl<'a> Thread<'a> {
    pub fn new(p: usize, c: String, t: &'a ThreadPool) -> Self {
        Thread {
            pid: p,
            cmd: c,
            parent: t,
        }
    }

    pub fn skill(self) {
        self.parent.is_dropped(self.pid);
    }
}

impl Drop for Thread<'_> {
    fn drop(&mut self) {
        self.parent.drop_thread(self.pid);
    }
}
