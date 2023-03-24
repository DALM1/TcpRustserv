use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn handle_client(mut stream: TcpStream, password: &str, users: Arc<Mutex<HashMap<TcpStream, String>>>) {
    let mut data = [0 as u8; 1024];
    
    // Demander le mot de passe
    stream.write(b"Entrez le mot de passe: ").unwrap();
    let size = stream.read(&mut data).unwrap();
    let password_input = std::str::from_utf8(&data[0..size]).unwrap().trim();

    // Vérifier le mot de passe
    if password_input != password {
        stream.write(b"Mot de passe incorrect").unwrap();
        return;
    }

    // Demander le nom d'utilisateur
    stream.write(b"Entrez votre nom d'utilisateur: ").unwrap();
    let size = stream.read(&mut data).unwrap();
    let username = std::str::from_utf8(&data[0..size]).unwrap().trim();

    // Confirmer la connexion
    stream.write(b"Connexion réussie").unwrap();

    // Ajouter l'utilisateur à la liste
    let mut users = users.lock().unwrap();
    users.insert(stream.try_clone().unwrap(), String::from(username));

    // Diffuser le message de connexion
    let message = format!("{} a rejoint le chat\n", username);
    broadcast_message(&stream, &message, &users);

    // Attendre les messages de l'utilisateur
    loop {
        let mut data = [0 as u8; 1024];
        let size = stream.read(&mut data).unwrap();
        let message = std::str::from_utf8(&data[0..size]).unwrap().trim();

        // Quitter le chat si l'utilisateur envoie la commande /quit
        if message == "/quit" {
            break;
        }

        // Diffuser le message de l'utilisateur
        let message = format!("{}: {}\n", username, message);
        broadcast_message(&stream, &message, &users);
    }

    // Retirer l'utilisateur de la liste
    users.remove(&stream.try_clone().unwrap());

    // Diffuser le message de déconnexion
    let message = format!("{} a quitté le chat\n", username);
    broadcast_message(&stream, &message, &users);
}

fn broadcast_message(sender: &TcpStream, message: &str, users: &Arc<Mutex<HashMap<TcpStream, String>>>) {
    let users = users.lock().unwrap();
    for client in users.keys() {
        if client.peer_addr().unwrap() != sender.peer_addr().unwrap() {
            client.write(message.as_bytes()).unwrap();
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5000")?;
    let password = "my_secret_password".to_string();
    let users = Arc::new(Mutex::new(HashMap::new()));
    println!("Serveur en attente de connexions sur le port 5000...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let password = password.clone();
                let users = users.clone();
                thread::spawn(move || {
                    handle_client(stream, &password, users);
                });
            }
            Err(e) =>
