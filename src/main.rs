fn handle_client_connection(mut stream: TcpStream) {
    
    let mut data = [0 as u8; 1024];
    stream.write(b"Entrez le mot de passe: ").unwrap();
    let size = stream.read(&mut data).unwrap();
    let password = std::str::from_utf8(&data[0..size]).unwrap().trim();

    
    if password != "my_secret_password" {
        stream.write(b"Mot de passe incorrect").unwrap();
        return;
    }

    
    stream.write(b"Entrez votre nom d'utilisateur: ").unwrap();
    let size = stream.read(&mut data).unwrap();
    let username = std::str::from_utf8(&data[0..size]).unwrap().trim();

    stream.write(b"Connexion réussie").unwrap();

   
    let mut users = USERS.lock().unwrap();
    users.insert(stream.try_clone().unwrap(), String::from(username));

   
    let message = format!("{} a rejoint le chat\n", username);
    broadcast_message(&stream, &message);

    
    loop {
        let mut data = [0 as u8; 1024];
        let size = stream.read(&mut data).unwrap();
        let message = std::str::from_utf8(&data[0..size]).unwrap().trim();

        if message == "/quit" {
            break;
        }

        let message = format!("{}: {}\n", username, message);
        broadcast_message(&stream, &message);
    }

    
    users.remove(&stream.try_clone().unwrap());
    let message = format!("{} a quitté le chat\n", username);
    broadcast_message(&stream, &message);
}



fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5000")?;
    let password = "my_secret_password".to_string(); // définir un mot de passe
    let mut clients = HashMap::new();
    println!("Serveur en attente de connexions sur le port 5000...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let password = password.clone();
                let mut clients = clients.clone();
                thread::spawn(move || {
                    handle_client(stream, &password, &mut clients);
                });
            }
            Err(e) => {
                println!("Erreur lors de la connexion : {}", e);
            }
        }
    }

    Ok(())
}
