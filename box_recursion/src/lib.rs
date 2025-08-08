#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(role: &str) -> Self {
        match role {
            "CEO" => Role::CEO,
            "Manager" => Role::Manager,
            "Normal Worker" => Role::Worker,
            _ => panic!("Unknown role"),
        }
    }
}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug, Clone)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        WorkEnvironment {
            grade: None,
        }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let mut new_worker = Worker {
            role: Role::from(role),
            name: name.to_string(),
            next: None,
        };

        match &self.grade {
            None => {
                self.grade = Some(Box::new(new_worker));
            }
            Some(worker) => {
                new_worker.next = Some(worker.clone());
                self.grade = Some(Box::new(new_worker));
            }
        }
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        match self.grade {
            None => None,
            Some(ref mut worker) => {
                let removed_worker = worker.clone();
                self.grade = worker.next.take();
                Some(removed_worker.name)
            }
        }
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        let current = &self.grade;
        match current {
            None => None,
            Some(_) => {
                Some((current.as_ref().unwrap().name.clone(), current.as_ref().unwrap().role.clone()))
            }
            
        }
    }
}