use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str;
use std::time::Duration;

fn main() {
    let mut stream: TcpStream;

    loop {
        let ip = "127.0.0.1:7777".parse().unwrap();
        let result = TcpStream::connect_timeout(&ip, Duration::from_secs(1));
        match result {
            Ok(tcpStream) => {
                stream = tcpStream;
                break;
            }
            Err(err) => {
                println!("connecting")
            }
        }
    }

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let result = stream.write(input.as_bytes());
        match result {
            Ok(sz) => { 
                if sz == 0 {
                    break;
                }
            },
            Err(err) => {}
        }
    }

    match stream.shutdown(std::net::Shutdown::Both) {
        Err(err) => {
            println!("{}", err.to_string());
            panic!();
        },
        _ => {}
    }
}
