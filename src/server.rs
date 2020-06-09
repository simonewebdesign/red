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
                println!("ENTIRE BUFFER = {:#?}", &buf);
                let iter = buf.split(|c| *c == 10);

                for bytes in iter {
                    handle_buf_slice(bytes, &stream, state);
                }

                break;
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // TODO: handle idle waiting for fd for linux
            }
            Err(e) => panic!("encountered IO error: {}", e),
        };
    };
}

fn handle_buf_slice(bytes: &[u8], mut stream: &TcpStream, state: &mut State) {
    match bytes {
        // GET key
        [103, 101, 116, ..] => {
            let (_, key) = bytes.split_at(4);

            match state.get(str::from_utf8(&key).unwrap()) {
                Some(value) => {
                    let _ = stream.write(value.as_bytes());
                    let _ = stream.write(&[10]);
                },
                None => {
                    let _ = stream.write(&[110, 105, 108, 10]); // nil
                }
            }
        }

        // SADD member
        [115, 97, 100, 100, ..] => {
            state.sadd("BOH".to_string());
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
            let (_, rhs) = bytes.split_at(4);
            let mut iter = rhs.split(|c| *c == 32); // space
            let key = iter.next().unwrap();
            let val = iter.next().unwrap();
            state.set(
                String::from_utf8(key.to_vec()).unwrap(),
                String::from_utf8(val.to_vec()).unwrap()
            );
            let _ = stream.write(&[79, 75, 10]); // OK
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
