use std::io::{self, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::mem;

fn main() {
    let stream: TcpStream;
    unsafe{
        stream = mem::uninitialized();
    }
    let stream = Arc::new(Mutex::new(stream));
    let stream_clone = stream.clone();
    let handle = thread::spawn(move || loop {
        let ip = "127.0.0.1:7777".parse().unwrap();
        let result = TcpStream::connect_timeout(&ip, Duration::from_secs(1));
        match result {
            Ok(stream) => {
                let mut s = stream_clone.lock().unwrap();
                *s = stream;
                break;
            }
            Err(_) => {
                println!("connecting");
            }
        }
    });
    let _ = handle.join();

    let mut stream_mut = stream.as_ref().lock().unwrap();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let result = stream_mut.write(input.as_bytes());
        match result {
            Ok(sz) => {
                if sz == 0 {
                    break;
                }
            }
            Err(_) => {}
        }
    }

    match stream_mut.shutdown(std::net::Shutdown::Both) {
        Err(err) => {
            println!("{}", err.to_string());
            panic!();
        }
        _ => {}
    }
}
