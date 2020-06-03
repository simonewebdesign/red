// use std::collections::HashMap;

// pub struct Store<'a> {
//     _data: &'a mut HashMap<&'a str, String>,
// }

// impl Store<'_> {
//     pub fn new() -> Store<'static> {
//         Store {
//             _data: HashMap::new(),
//         }
//     }

//     pub fn get(&self, key: &str) -> String {
//         return match self._data.get(&key) {
//             Some(&value) => value.to_string(),
//             _ => "(nil)".to_string(),
//         }
//     }

//     pub fn set(&mut self, key: &'static str, val: &str) {
//         self._data.insert(key, val.to_string());
//     }
// }
