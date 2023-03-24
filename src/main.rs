use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    stream.write_all(b"Enter your username: ").unwrap();
    stream.flush().unwrap();

    let mut username = String::new();
    stream.read_line(&mut username).unwrap();
    let username = username.trim();

    stream.write_all(b"Enter the password: ").unwrap();
    stream.flush().unwrap();

    let mut password = String::new();
    stream.read_line(&mut password).unwrap();
    let password = password.trim();

    stream.write_all(b"Welcome!\n").unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream);
    }
}
