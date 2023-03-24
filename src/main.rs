use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let password = "password";
    let clients: HashMap<String, TcpStream> = HashMap::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let clients = clients.clone();
                thread::spawn(move || {
                    handle_client(stream, password, &mut clients.lock().unwrap());
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, password: &str, clients: &mut HashMap<String, TcpStream>) {
    let mut buffer = [0; 1024];
    let mut username = String::new();

    stream.write(b"Enter a username: ").unwrap();
    stream.read(&mut buffer).unwrap();
    username.push_str(&String::from_utf8_lossy(&buffer[..]).trim());

    if username == password {
        stream.write(b"Invalid username.\n").unwrap();
        return;
    }

    clients.insert(username.clone(), stream.try_clone().unwrap());
    let welcome_message = format!("Welcome, {}!", username);
    broadcast_message(clients, &welcome_message);

    loop {
        let mut buffer = [0; 1024];
        let nbytes = stream.read(&mut buffer).unwrap();
        if nbytes == 0 {
            let goodbye_message = format!("{} has left the chat.", username);
            clients.remove(&username);
            broadcast_message(clients, &goodbye_message);
            break;
        }
        let message = String::from_utf8_lossy(&buffer[..nbytes]).trim().to_string();
        let message = format!("{}: {}", username, message);
        broadcast_message(clients, &message);
    }
}

fn broadcast_message(clients: &HashMap<String, TcpStream>, message: &str) {
    for client in clients.values() {
        client.write(message.as_bytes()).unwrap();
        client.write(b"\n").unwrap();
    }
}
