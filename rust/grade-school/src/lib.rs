use std::collections::HashMap;

pub struct School {
    students_by_grade: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School { students_by_grade: HashMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if !self.students_by_grade.contains_key(&grade) {
            self.students_by_grade.insert(grade, Vec::new());
        }
        let students = self.students_by_grade.get_mut(&grade).unwrap();
        students.push(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut keys : Vec<u32> = self.students_by_grade.keys().cloned().collect();
        keys.sort();
        keys
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        match self.students_by_grade.get(&grade) {
            Some(v) => {
                let mut v = v.clone();
                dbg!(&v);
                v.sort();
                Some(v)
            },
            None => None,
        }
    }
}
