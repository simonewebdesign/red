extern crate signal_hook;

use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::str;
use std::path::Path;
mod lib;
use lib::State;
use std::env;
// use std::io;
// use std::io::prelude::*;
// use std::fs;
// use std::fs::File;
// use std::io::Error;
// use std::path::Path;
// use std::process;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use signal_hook::flag as signal_flag;
// mod lib;

fn main() {
    let mut args = env::args().skip(1);
    let mut host = "127.0.0.1".to_string();
    let mut port = "7878".to_string();

    loop {
        match args.next() {
            Some(x) if x == "--host" => {
                host = args.next().unwrap_or(host);
            },
            Some(x) if x == "--port" => {
                port = args.next().unwrap_or(port);
            },
            Some(x) => {
                println!("unknown argument: {}", x);
            }
            None => {
                break;
            }
        }
    }

    let listener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");

    let mut state = if Path::new("store.red").exists() {
        State::deserialize(read_file())
    } else {
        State::new()
    };

    let term = Arc::new(AtomicUsize::new(0));
    const SIGTERM: usize = signal_hook::SIGTERM as usize;
    const SIGINT: usize = signal_hook::SIGINT as usize;
    const SIGQUIT: usize = signal_hook::SIGQUIT as usize;
    signal_flag::register_usize(signal_hook::SIGTERM, Arc::clone(&term), SIGTERM)?;
    signal_flag::register_usize(signal_hook::SIGINT, Arc::clone(&term), SIGINT)?;
    signal_flag::register_usize(signal_hook::SIGQUIT, Arc::clone(&term), SIGQUIT)?;

    loop {
        match term.load(Ordering::Relaxed) {
            0 => {
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
            SIGTERM => {
                eprintln!("Terminating on the TERM signal");
                // break;
            }
            SIGINT => {
                eprintln!("Terminating on the INT signal");
                // break;
            }
            SIGQUIT => {
                eprintln!("Terminating on the QUIT signal");
                // break;
            }
            _ => unreachable!(),
        }
    }
}

fn handle_conn(mut stream: TcpStream, state: &mut State) {
    let mut buf = vec![];
    loop {
        match stream.read_to_end(&mut buf) {
            Ok(_) => {
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
            let (_, rhs) = bytes.split_at(5);
            state.sadd(
                String::from_utf8(rhs.to_vec()).unwrap(),
            );
        }
        // SMEMBERS
        [115, 109, 101, 109, 98, 101, 114, 115, ..] => {
            for member in state.smembers() {
                let _ = stream.write(member.as_bytes());
                let _ = stream.write(&[10]);
            }
        }
        // SREM member
        [115, 114, 101, 109, ..] => {
            let (_, rhs) = bytes.split_at(5);
            state.srem(str::from_utf8(&rhs).unwrap());
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

        [] => {
            // Reached end of stream.
        }

        _ => {
            println!("unknown operation");
        }
    }
}

fn read_file() -> String {
    fs::read_to_string("store.red")
        .expect("Failed reading from file")
}
