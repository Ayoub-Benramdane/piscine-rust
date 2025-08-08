#[derive(Debug, PartialEq)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        todo!()
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        todo!()
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        todo!()
    }
}