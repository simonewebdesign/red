use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub fn init() -> std::io::Result<()> {
    let mut store = File::create("redstore")?;
    store.write_all(b"11111 foo@example.com\n22222 bar@example.com\n33333 baz@example.com\n")?;
    Ok(())
}

pub fn read_store() {
    let contents = fs::read_to_string("redstore")
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
