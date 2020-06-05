use std::io;
use std::collections::{HashMap, HashSet};

// My choices: from https://stackoverflow.com/questions/59973669/is-it-possible-to-have-a-hashmap-that-can-accept-values-of-multiple-types-or-of
// 1. Model your value type as a enum and insert that into hashmap. – edwardw Jan 29 at 18:49
// 2. Or find a common trait and use trait objects. – Shepmaster Jan 29 at 19:01
// I tried 1.:
#[derive(Debug)]
pub enum Value {
    Str(String),
    HSet(HashSet<String>),
}
// It seems cumbersome.
// Also it doesnt work right away. I get these errors:
// `Value` doesn't implement `std::fmt::Debug`
// `Value` cannot be formatted using `{:?}`
// help: the trait `std::fmt::Debug` is not implemented for `Value`
// note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
// note: required because of the requirements on the impl of `std::fmt::Debug` for `&Value`
// note: required by `std::fmt::Debug::fmt`rustc(E0277)

// Seems like you were on the right track initially. You simply want to say that an abstract type
// will resolve to either A or B concrete type.

fn main() {
    let mut store = HashMap::new();

    loop {
        read_eval_print(&mut store);
    }
}

pub fn read_eval_print(store: &mut HashMap<String, Value>) {
    print!("> ");
    io::Write::flush(&mut io::stdout())
        .expect("flush failed");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .ok()
        .expect("failed to read line");

    let command_with_args: Vec<&str> = input.split(" ").collect();

    match command_with_args.as_slice() {
        ["get", key] => {
            match store.get(key.trim()) {
                Some(value) => println!("{:?}", value),
                None => println!("(nil)")
            }
        }

        ["sadd", key, member] => {
            let mut set: HashSet<String> = HashSet::new();
            set.insert(member.trim().to_string());
            &store.insert(key.to_string(), Value::HSet(set));
            println!("OK");
        }

        ["set", key, val] => {
            &store.insert(key.to_string(), Value::Str(val.trim().to_string()));
            println!("OK");
        }

        ["debug\n"] | ["debug", ..] => {
            println!("store = {:#?}", store);
        }

        _ => {
            println!("ERR unknown command");
        }
    }
}
