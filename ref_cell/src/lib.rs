use std::rc::Rc;

pub struct Tracker {
    pub max: usize,
    pub messages: std::cell::RefCell<Vec<String>>,
    pub value: usize
}

impl Tracker {
    pub fn new(max: usize) -> Self {
        Tracker {
            max,
            messages: std::cell::RefCell::new(Vec::new()),
            value: 0,
        }
    }

    pub fn peek(&self, rc: &Rc<i32>) {
        let current = Rc::strong_count(rc);
        self.messages.borrow_mut().push(format!(
            "Info: This value would use {}% of your quota",
            ((current as f64 / self.max as f64) * 100.0) as i32
        ));
    }

    pub fn set_value(&self, rc: &Rc<i32>) {
        let current = Rc::strong_count(rc);
        if current > self.max {
            self.messages.borrow_mut().push(format!(
                "Error: You can't go over your quota!"
            ));
        } else {
            self.messages.borrow_mut().push(format!(
                "Warning: You have used up over {}% of your quota!",
                ((current as f64 / self.max as f64) * 100.0) as i32
            ));
        }
    }
}