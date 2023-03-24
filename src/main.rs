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

    
    stream.write(b"Connexion réussie.\n").unwrap();

    loop {
       
        let mut data = [0 as u8; 1024];
        let size = stream.read(&mut data).unwrap();
        let message = std::str::from_utf8(&data[0..size]).unwrap().trim();

       
        if message == "/quit" {
            let goodbye_message = format!("{} a quitté le chat.\n", username);
            broadcast_message(clients, &goodbye_message);
            clients.remove(&String::from(username));
            break;
        }

       
        let message = format!("{}: {}\n", username, message);
        broadcast_message(clients, &message);
    }
}

Ok(())
}
