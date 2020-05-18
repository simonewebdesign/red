use std::process;

mod lib;

fn main() {
    if let Err(_) = lib::init() {
        println!("red: error: failed to initialize store");
        process::exit(1);
    }
    lib::read_store();

    loop {
        lib::read_and_print();
    }
}
