use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn init() -> std::io::Result<()> {
    let mut store = File::create("redstore")?;
    store.write_all(b"11111 foo@example.com\n22222 bar@example.com\n33333 baz@example.com\n")?;
    Ok(())
}

fn read_store() -> String {
    let contents = fs::read_to_string("redstore")
        .expect("Something went wrong reading the file");

    return contents;
}

pub fn read_eval_print() {
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
            println!("User wants to get value for key {}", key);
            let value = find_in_store(key);
            println!("{}", value);
        }

        ["set", key, val] => {
            println!("User wants to set value {} for key {}", val, key);
        }

        _ => {
            println!("Unknown command");
        }
    }
}

fn find_in_store(key: &str) -> String {
    let contents = read_store();

    for row in contents.split("\n") {
        let vec: Vec<&str> = row.split(" ").collect();

        if vec[0] == key {
            return vec[1].to_string();
        }
    }
    return "(nil)".to_string();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
