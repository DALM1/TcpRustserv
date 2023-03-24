use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();
    let password = "my_secret_password";

    let clients: HashMap<String, TcpStream> = HashMap::new();
    let clients = std::sync::Mutex::new(clients);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let clients = clients.clone();
                thread::spawn(move || {
                    handle_client(stream, password, &clients);
                });
            }
            Err(e) => {
                eprintln!("failed to connect: {}", e);
            }
        }
    }
}

fn handle_client(
    mut stream: TcpStream,
    password: &str,
    clients: &std::sync::Mutex<HashMap<String, TcpStream>>,
) {
    let mut reader = BufReader::new(&stream);
    let mut username = String::new();

  
    loop {
        stream.write_all(b"Enter your username: ").unwrap();
        stream.flush().unwrap();

        reader.read_line(&mut username).unwrap();

        if username.trim().is_empty() {
            continue;
        }

        let mut clients = clients.lock().unwrap();

        if clients.contains_key(&username) {
            stream
                .write_all(b"Username is already taken. Please try again.\n")
                .unwrap();
            stream.flush().unwrap();
            username.clear();
        } else {
            clients.insert(username.trim().to_string(), stream.try_clone().unwrap());
            break;
        }
    }

    loop {
        stream.write_all(b"Enter the password: ").unwrap();
        stream.flush().unwrap();

        let mut password_attempt = String::new();
        reader.read_line(&mut password_attempt).unwrap();

        if password_attempt.trim() != password {
            stream.write_all(b"Incorrect password. Please try again.\n").unwrap();
            stream.flush().unwrap();
        } else {
            stream.write_all(b"Welcome!\n").unwrap();
            stream.flush().unwrap();
            break;
        }
    }


    loop {
        let mut message = String::new();
        reader.read_line(&mut message).unwrap();

        if message.trim() == "/quit" {
            break;
        }

        let mut clients = clients.lock().unwrap();
        let mut to_remove = Vec::new();

        for (client_username, client_stream) in clients.iter_mut() {
            if *client_stream != stream {
                let response = format!("{}: {}", username.trim(), message.trim());
                client_stream.write_all(response.as_bytes()).unwrap();
                client_stream.flush().unwrap();
            } else {
                to_remove.push(client_username.clone());
            }
        }

        for client_username in to_remove {
            clients.remove(&client_username);
        }
    }

    let mut clients = clients.lock().unwrap();
    clients.remove(&username);
}
