use std::io;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;
mod lib;
use lib::State;

fn main() {
    let mut state;

    if Path::new("store.red").exists() {
        state = State::deserialize(restore());
    } else {
        state = State::new();
    }

    loop {
        read_eval_print(&mut state);
    }
}

pub fn read_eval_print(state: &mut State) {
    print!("> ");
    io::Write::flush(&mut io::stdout())
        .expect("flush failed");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("failed to read line");

    let command_with_args: Vec<&str> = input.split(' ').collect();

    match command_with_args.as_slice() {
        ["save\n"] => {
            match save(state.serialize()) {
                Ok(_) => {
                    println!("Saving completed");
                }
                Err(msg) => {
                    println!("Saving failed: {}", msg);
                }
            }
        }

        ["get", key] => {
            match state.get(key.trim()) {
                Some(value) => println!("{}", value),
                None => println!("(nil)")
            }
        }

        ["sadd", member] => {
            state.sadd(member.trim().to_string());
            println!("OK");
        }

        ["smembers\n"] => {
            for member in state.smembers() {
                println!("{}", member);
            }
        }

        ["srem", member] => {
            state.srem(member.trim());
        }

        ["set", key, val] => {
            state.set(key.to_string(), val.trim().to_string());
            println!("OK");
        }

        ["debug\n"] => {
            println!("{:#?}", state);
        }

        ["ser\n"] => {
            println!("{}", state.serialize());
        }

        ["des\n"] => {
            println!("{:#?}", State::deserialize(state.serialize()));
        }

        _ => {
            println!("ERR unknown command");
        }
    }
}

fn save(data: String) -> std::io::Result<()> {
    let mut store = File::create("store.red")?;
    store.write_all(data.as_bytes())
}

fn restore() -> String {
    fs::read_to_string("store.red")
        .expect("Failed restoring state")
}
