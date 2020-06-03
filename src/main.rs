// use std::process;
use std::io;
use std::collections::HashMap;

mod lib;

fn main() {
    // if let Err(_) = lib::init() {
    //     println!("red: error: failed to initialize store");
    //     process::exit(1);
    // }

    let mut store: HashMap<String, String> = HashMap::new();
    store.insert(
        "foo".to_string(),
        "bar".to_string(),
    );

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
            println!("# BEGIN STORE, right after an insertion #");
            for (key, value) in store {
                println!("{}: {}", key, value);
            }
            // println!("normal: {:?} pretty: {:#?}", store, store);
            println!("# END STORE, right after an insertion #");
            println!("OK");
        }

        ["debug", ..] => {
            println!("# BEGIN STORE #");
            for (key, value) in store {
                println!("{}: {}", key, value);
            }
            // println!("normal: {:?} pretty: {:#?}", store, store);
            println!("# END STORE #");
        }

        _ => {
            println!("ERR unknown command");
        }
    }

    println!("Debug: command was: {:?}", command_with_args.as_slice())
}
