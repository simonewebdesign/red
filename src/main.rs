use std::io;
use std::collections::{HashMap, HashSet};
use std::fmt;

fn main() {
    let mut store = HashMap::new();
    let mut set = HashSet::new();

    loop {
        read_eval_print(&mut store, &mut set);
    }
}

pub fn read_eval_print(
    store: &mut HashMap<String, String>,
    set: &mut HashSet<String>,
) {
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

        ["sadd", member] => {
            set.insert(member.trim().to_string());
            println!("OK");
        }

        ["smembers\n"] => {
            for member in set.iter() {
                println!("{}", member);
            }
        }

        ["srem", member] => {
            set.remove(member.trim());
        }

        ["set", key, val] => {
            &store.insert(key.to_string(), val.trim().to_string());
            println!("OK");
        }

        ["debug\n"] => {
            println!("store = {:#?}", store);
            println!("set = {:#?}", set);
        }

        _ => {
            println!("ERR unknown command");
        }
    }
}
