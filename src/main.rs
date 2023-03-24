use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;

type UserList = Arc<Mutex<HashMap<TcpStream, String>>>;

fn handle_client(mut stream: TcpStream, password: &str, users: &UserList) {
    let mut data = [0 as u8; 1024];
    
    // Demande du mot de passe
    stream.write(b"Entrez le mot de passe: ").unwrap();
    let size = stream.read(&mut data).unwrap();
    let password_input = std::str::from_utf8(&data[0..size]).unwrap().trim();

    // Vérification du mot de passe
    if password_input != password {
        stream.write(b"Mot de passe incorrect").unwrap();
        return;
    }

    // Demande du nom d'utilisateur
    stream.write(b"Entrez votre nom d'utilisateur: ").unwrap();
    let size = stream.read(&mut data).unwrap();
    let username = std::str::from_utf8(&data[0..size]).unwrap().trim();

    // Connexion réussie
    stream.write(b"Connexion réussie\n").unwrap();

    // Ajout de l'utilisateur à la liste des utilisateurs connectés
    let mut users = users.lock().unwrap();
    users.insert(stream.try_clone().unwrap(), String::from(username));

    // Diffusion d'un message informant que l'utilisateur s'est connecté
    let message = format!("{} a rejoint le chat\n", username);
    broadcast_message(&stream, &message, &users);

    // Boucle de lecture des messages envoyés par l'utilisateur
    loop {
        let mut data = [0 as u8; 1024];
        let size = stream.read(&mut data).unwrap();
        let message = std::str::from_utf8(&data[0..size]).unwrap().trim();

        // Arrêt du traitement si l'utilisateur a saisi "/quit"
        if message == "/quit" {
            break;
        }

        // Diffusion du message de l'utilisateur aux autres utilisateurs
        let message = format!("{}: {}\n", username, message);
        broadcast_message(&stream, &message, &users);
    }

    // Retrait de l'utilisateur de la liste des utilisateurs connectés
    users.remove(&stream.try_clone().unwrap());

    // Diffusion d'un message informant que l'utilisateur s'est déconnecté
    let message = format!("{} a quitté le chat\n", username);
    broadcast_message(&stream, &message, &users);
}

fn broadcast_message(sender: &TcpStream, message: &str, users: &UserList) {
    let mut to_remove = Vec::new();

    for (client, _) in users.lock().unwrap().iter() {
        if client.peer_addr().unwrap() != sender.peer_addr().unwrap() {
            if let Err(_) = client.write(message.as_bytes()) {
                to_remove.push(client.clone());
            }
        }
    }

    let mut users = users.lock().unwrap();
    for client in to_remove {
        users.remove(&client);
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5000")?;
    let password = "my_secret_password".to_string(); // Définition du mot de passe
    let users = Arc::new(Mutex::new(HashMap::new()));

    println!("Serveur en attente de connexions sur le
