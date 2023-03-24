use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;


type UserList = Arc<Mutex<HashMap<TcpStream, String>>>;

fn handle_client(mut stream: TcpStream, password: &str, clients: &mut HashMap<String, TcpStream>) {
    let mut data = [0 as u8; 1024];

    stream.write(b"Entrez le mot de passe: ").unwrap();
    let size = stream.read(&mut data).unwrap();
    let password_input = std::str::from_utf8(&data[0..size]).unwrap().trim();

    if password_input != password {
        stream.write(b"Mot de passe incorrect").unwrap();
        return;
    }

    stream.write(b"Entrez votre nom d'utilisateur: ").unwrap();
    let size = stream.read(&mut data).unwrap();
    let username = std::str::from_utf8(&data[0..size]).unwrap().trim();

    clients.insert(String::from(username), stream.try_clone().unwrap());
    stream.write(b"Connexion reussie.\n").unwrap();

    loop {
        let mut data = [0 as u8; 1024];
        let size = stream.read(&mut data).unwrap();
        let message = std::str::from_utf8(&data[0..size]).unwrap().trim();

        if message == "/quit" {
            let goodbye_message = format!("{} a quitte le chat.\n", username);
            broadcast_message(clients, &goodbye_message);
            clients.remove(&String::from(username));
            break;
        }

        let message = format!("{}: {}\n", username, message);
        broadcast_message(clients, &message);
    }
}

fn broadcast_message(clients: &mut HashMap<String, TcpStream>, message: &str) {
    let mut to_remove = vec![];

    for (username, client) in clients.iter_mut() {
        let message_bytes = message.as_bytes();
        if client.write(message_bytes).is_err() {
            to_remove.push(username.clone());
        }
    }

    for username in to_remove {
        clients.remove(&username);
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:12345").unwrap();
    let password = "password";
    let clients: UserList = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let clients = clients.clone();
                let password = password.to_owned();

                thread::spawn(move || {
                    handle_client(stream, &password, &mut clients.lock().unwrap());
                });
            }
            Err(e) => {
                println!("Erreur de connexion: {}", e);
            }
        }
    }
}
