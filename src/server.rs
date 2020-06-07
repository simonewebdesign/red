use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                handle_conn(s);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => panic!("Encountered IO error: {}", e),
        }
    }
}

fn handle_conn(mut stream: TcpStream) {
    let mut buf = vec![];
    loop {
        match stream.read_to_end(&mut buf) {
            Ok(_) => {
                handle_bytes(&buf);
                break;
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // TODO: handle idle waiting for fd for linux
            }
            Err(e) => panic!("encountered IO error: {}", e),
        };
    };
}

fn handle_bytes(mut buffer: &[u8]) {
    println!("result: {:?}", buffer);
    match buffer {
        // SET key value
        [115, 101, 116, ..]  => {
            println!("it's a set operation");
        }

        _ => {
            println!("unknown operation");
        }
    }
}
