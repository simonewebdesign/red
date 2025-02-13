use std::collections::{HashMap, HashSet};

/// An opaque type that stores data.
///
/// This is used by both red and red-server independently, kind of like a
/// singleton instance; however, you can make as many instances as you need.
#[derive(Debug, PartialEq)]
pub struct State {
    store: HashMap<String, String>,
    set: HashSet<String>,
}

/// Implements functionality for `Store`. At time of writing this documentation,
/// it supports a few "commands" in a similar way to Redis: I've only
/// implemented the ones I actually need.
///
/// Full 1:1 mapping of Redis commands is not a goal, but it would be nice to
/// keep following their spec closely.
impl State {
    /// Instantiate `State` by calling this function.
    ///
    /// # Example
    ///
    /// ```
    /// use red::State;
    ///
    /// let mut state = State::new();
    /// ```
    pub fn new() -> Self {
        State {
            store: HashMap::new(),
            set: HashSet::new(),
        }
    }

    /// Get the value of key.
    ///
    /// # Example
    ///
    /// ```
    /// // The return value of the function is an option
    /// # let mut state = red::State::new();
    /// let value = state.get("someKey");
    ///
    /// // Pattern match to retrieve the value
    /// match value {
    ///     // The key was present
    ///     Some(v) => println!("Value: {}", v),
    ///     // The key was not present
    ///     None    => println!("Nope"),
    /// }
    /// ```
    pub fn get(&mut self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    /// Add a new member to the set.
    /// If the member is already in the set, it will be ignored.
    ///
    /// # Example
    ///
    /// ```
    /// # let mut state = red::State::new();
    /// state.sadd("BOOMBAYAH".to_string());
    ///
    /// assert_eq!("BOOMBAYAH", state.smembers().next().unwrap());
    /// ```
    pub fn sadd(&mut self, member: String) -> bool {
        self.set.insert(member)
    }

    /// Returns an iterator to retrieve some or all the members of the set.
    /// The retrieval order is completely arbitrary.
    ///
    /// # Example
    ///
    /// ```
    /// # let mut state = red::State::new();
    /// state.sadd("foo".to_string());
    /// state.sadd("bar".to_string());
    ///
    /// // Will print in an arbitrary order.
    /// for member in state.smembers() {
    ///     println!("{}", member);
    /// }
    /// ```
    /// For more detail about iterators, see the
    /// [official docs](https://doc.rust-lang.org/std/iter/index.html).
    pub fn smembers(&mut self) -> impl Iterator<Item = &String> {
        self.set.iter()
    }

    /// Remove the specified member from the set.
    ///
    /// Returns true if the member was in the set; false otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// # let mut state = red::State::new();
    /// state.sadd("mittsies".to_string());
    ///
    /// assert_eq!(state.srem("mittsies"), true);
    /// assert_eq!(state.srem("mittsies"), false);
    /// ```
    pub fn srem(&mut self, member: &str) -> bool {
        self.set.remove(member)
    }

    /// Set key to hold the specified value. If the key already holds a value,
    /// it is overwritten.
    ///
    /// # Example
    ///
    /// ```
    /// # let mut state = red::State::new();
    /// state.set("someKey".to_string(), "Blade".to_string());
    ///
    /// assert_eq!(state.get("someKey").unwrap(), "Blade");
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    /// Serializes Self into a String.
    ///
    /// # Example
    /// ```
    /// # let mut state = red::State::new();
    /// state.sadd("a".to_string());
    /// state.sadd("b".to_string());
    ///
    /// state.set("x".to_string(), "1".to_string());
    /// state.set("y".to_string(), "2".to_string());
    ///
    /// let s = state.serialize();
    ///
    /// assert!(s.contains("\na"));
    /// assert!(s.contains("\nb"));
    /// assert!(s.contains("\t"));
    /// assert!(s.contains("\nx 1"));
    /// assert!(s.contains("\ny 2"));
    pub fn serialize(&mut self) -> String {
        let mut res = String::new();

        for member in self.set.iter() {
            res += "\n";
            res += member;
        }
        res += "\t";

        for (key, val) in &self.store {
            res += "\n";
            res += key;
            res += " ";
            res += val;
        }
        res
    }

    /// Deserializes a previously serialized String into Self.
    ///
    /// # Example
    /// ```
    /// # let mut state = red::State::new();
    /// state.sadd("a".to_string());
    /// state.sadd("b".to_string());
    ///
    /// state.set("x".to_string(), "1".to_string());
    /// state.set("y".to_string(), "2".to_string());
    ///
    /// let s = red::State::deserialize(state.serialize());
    ///
    /// assert_eq!(state, s);
    pub fn deserialize(string: String) -> Self {
        let mut set = HashSet::new();
        let mut store = HashMap::new();
        let mut iter = string.split('\t');

        for elem in iter.next().unwrap().split('\n').skip(1) {
            set.insert(elem.to_string());
        }

        for kv in iter.next().unwrap().split('\n').skip(1) {
            let mut pair = kv.split(' ');
            store.insert(pair.next().unwrap().to_string(), pair.next().unwrap().to_string());
        }

        State {
            set,
            store,
        }
    }
}
