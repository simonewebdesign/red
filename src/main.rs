use std::io;
use std::collections::HashMap;

fn main() {
    let mut store: HashMap<String, String> = HashMap::new();

    loop {
        read_eval_print(&mut store);
    }
}

pub fn read_eval_print(store: &mut HashMap<String, String>) {
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
                Some(value) => println!("{}", value),
                None => println!("(nil)")
            }
        }

        ["set", key, val] => {
            &store.insert(key.to_string(), val.trim().to_string());
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
