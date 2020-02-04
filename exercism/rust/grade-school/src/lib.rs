use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub struct School {
    students: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            students: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let names = self.students.entry(grade).or_default();
        names.insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.students.keys().map(|k| *k).collect()
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.students
            .get(&grade)
            .map(|set| set.into_iter().map(|s| s.to_string()).collect())
    }
}
