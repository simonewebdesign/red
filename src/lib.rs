use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct State {
    store: HashMap<String, String>,
    set: HashSet<String>,
}

impl State {
    pub fn new() -> Self {
        State {
            store: HashMap::new(),
            set: HashSet::new(),
        }
    }

    pub fn get(&mut self, key: &str) -> String {
        match self.store.get(key) {
            Some(value) => value.to_string(),
            None => "(nil)".to_string()
        }
    }

    pub fn sadd(&mut self, member: String) -> bool {
        self.set.insert(member)
    }

    pub fn smembers(&mut self) -> impl Iterator<Item = &String> {
        return self.set.iter();
    }

    pub fn srem(&mut self, member: &str) -> bool {
        self.set.remove(member)
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }
}
