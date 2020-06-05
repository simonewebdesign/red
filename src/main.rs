use std::io;
use std::collections::{HashMap, HashSet};
use std::fmt;

fn main() {
    let mut store = HashMap::new();

    loop {
        read_eval_print(&mut store);
    }
}

pub fn read_eval_print(store: &mut HashMap<String, Box<dyn fmt::Debug + 'static>>) {
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
            &store.insert(key.to_string(), Box::new(set));
            println!("OK");
        }

        ["set", key, val] => {
            &store.insert(key.to_string(), Box::new(val.trim().to_string()));
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
