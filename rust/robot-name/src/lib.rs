use rand::{thread_rng, Rng};
use std::cell::RefCell;
use std::collections::HashSet;

thread_local!(static ROBOT_NAMES: RefCell<HashSet<String>> = RefCell::new(HashSet::new()));

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: generate_name(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        self.name = generate_name();
    }
}

fn generate_name() -> String {
    ROBOT_NAMES.with(|store| {
        let mut set = store.borrow_mut();
        let mut rng = thread_rng();
        let mut name;
        while {
            name = String::new();
            name.push(rng.gen_range(b'A', b'Z' + 1) as char);
            name.push(rng.gen_range(b'A', b'Z' + 1) as char);
            name.push(rng.gen_range(b'0', b'9' + 1) as char);
            name.push(rng.gen_range(b'0', b'9' + 1) as char);
            name.push(rng.gen_range(b'0', b'9' + 1) as char);
            !set.insert(name.clone())
        } {}
        name
    })
}
