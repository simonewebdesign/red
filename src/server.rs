use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
mod lib;
use lib::State;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");

    let mut state = State::new();

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                handle_conn(s, &mut state);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => panic!("Encountered IO error: {}", e),
        }
    }
}

fn handle_conn(mut stream: TcpStream, state: &mut State) {
    let mut buf = vec![];
    loop {
        match stream.read_to_end(&mut buf) {
            Ok(_) => {
                handle_bytes(&buf, stream, state);
                break;
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // TODO: handle idle waiting for fd for linux
            }
            Err(e) => panic!("encountered IO error: {}", e),
        };
    };
}

fn handle_bytes(mut buffer: &[u8], mut stream: TcpStream, state: &mut State) {
    println!("result: {:?}", buffer);
    match buffer {
        // GET key
        [103, 101, 116, ..] => {
            match state.get("foo") {
                Some(value) => println!("{}", value),
                None => println!("(nil)")
            }
        }

        // SADD member
        [115, 97, 100, 100, ..] => {
            state.sadd("BOH".to_string());
            println!("OK");
        }

        // SMEMBERS
        [115, 109, 101, 109, 98, 101, 114, 115, ..] => {
            for member in state.smembers() {
                println!("{}", member);
            }
        }

        // SREM member
        [115, 114, 101, 109, ..] => {
            state.srem("BOH");
        }

        // SET key value
        [115, 101, 116, ..]  => {
            state.set("somekey".to_string(), "someValue".to_string());
            println!("it's a set operation. A sample key has been added to the state.");

            let _ = stream.write(&[79, 75]);
        }

        // DEBUG
        [100, 101, 98, 117, 103, ..] => {
            println!("{:#?}", state);
        }

        _ => {
            println!("unknown operation");
        }
    }
}
