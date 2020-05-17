use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    if let Err(_) = init() {
        println!("red: error: failed to initialize store");
        process::exit(1);
    }
}

fn init() -> std::io::Result<()> {
    let mut store = File::create("redstore")?;
    store.write_all(b"Hello, world!")?;
    Ok(())
}
