fn handle_client_connection(mut stream: TcpStream) {
    // Demande le mot de passe
    let mut data = [0 as u8; 1024];
    stream.write(b"Entrez le mot de passe: ").unwrap();
    let size = stream.read(&mut data).unwrap();
    let password = std::str::from_utf8(&data[0..size]).unwrap().trim();

    // Vérifie le mot de passe
    if password != "my_secret_password" {
        stream.write(b"Mot de passe incorrect").unwrap();
        return;
    }

    // Demande le nom d'utilisateur
    stream.write(b"Entrez votre nom d'utilisateur: ").unwrap();
    let size = stream.read(&mut data).unwrap();
    let username = std::str::from_utf8(&data[0..size]).unwrap().trim();

    stream.write(b"Connexion réussie").unwrap();

    // Ajoute l'utilisateur à la liste
    let mut users = USERS.lock().unwrap();
    users.insert(stream.try_clone().unwrap(), String::from(username));

    // Envoie le message de bienvenue
    let message = format!("{} a rejoint le chat\n", username);
    broadcast_message(&stream, &message);

    // Lit les messages entrants et les diffuse à tous les utilisateurs
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

    // Retire l'utilisateur de la liste et envoie un message de départ
    users.remove(&stream.try_clone().unwrap());
    let message = format!("{} a quitté le chat\n", username);
    broadcast_message(&stream, &message);
}
