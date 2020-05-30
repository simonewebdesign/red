use std::process;

mod lib;

fn main() {
    if let Err(_) = lib::init() {
        println!("red: error: failed to initialize store");
        process::exit(1);
    }

    loop {
        lib::read_eval_print();
    }
}
