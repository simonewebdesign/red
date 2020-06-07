use std::io;
mod lib;
use lib::State;

fn main() {
    let mut state = State::new();

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
        .ok()
        .expect("failed to read line");

    let command_with_args: Vec<&str> = input.split(" ").collect();

    match command_with_args.as_slice() {
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

        _ => {
            println!("ERR unknown command");
        }
    }
}
