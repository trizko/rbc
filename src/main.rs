use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::str;

fn handle_client(mut stream: TcpStream) {
    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                println!("{:?}", str::from_utf8(&read[0..n]));
                stream.write(&read[0..n]).unwrap();
            }
            Err(err) => {
                panic!(err);
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8333").unwrap();
    println!("Listening on port 8333...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
}
