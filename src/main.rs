use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    if let Err(_) = init() {
        println!("red: error: failed to initialize store");
        process::exit(1);
    }
    read_store();
}

fn init() -> std::io::Result<()> {
    let mut store = File::create("redstore")?;
    store.write_all(b"11111 foo@example.com\n22222 bar@example.com\n33333 baz@example.com\n")?;
    Ok(())
}

fn read_store() {
    let contents = fs::read_to_string("redstore")
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
