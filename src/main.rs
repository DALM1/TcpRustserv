use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn handle_client(stream: TcpStream, users: Arc<Mutex<Vec<String>>>) {
    let mut stream = BufReader::new(stream);
    let mut line = String::new();

   
    let password = "my_secret_password";
    write!(stream.get_mut(), "Enter password: ").unwrap();
    stream.get_mut().flush().unwrap();

   
    stream.read_line(&mut line).unwrap();
    let password_attempt = line.trim();

    
    if password_attempt != password {
        write!(stream.get_mut(), "Incorrect password\n").unwrap();
        return;
    }

   
    line.clear();
    write!(stream.get_mut(), "Enter username: ").unwrap();
    stream.get_mut().flush().unwrap();

   
    stream.read_line(&mut line).unwrap();
    let username = line.trim().to_owned();

    
    users.lock().unwrap().push(username.clone());

   
    let message = format!("{} has joined the chat\n", username);
    println!("{}", message);
    for user in users.lock().unwrap().iter() {
        let mut user_stream = stream.get_ref().try_clone().unwrap();
        write!(user_stream, "{}", message).unwrap();
        user_stream.flush().unwrap();
    }

   
    loop {
        line.clear();
        stream.read_line(&mut line).unwrap();
        let message = line.trim().to_owned();

      
        if message.is_empty() {
            let index = users.lock().unwrap().iter().position(|u| u == &username).unwrap();
            users.lock().unwrap().remove(index);
            let message = format!("{} has left the chat\n", username);
            println!("{}", message);
            for user in users.lock().unwrap().iter() {
                let mut user_stream = stream.get_ref().try_clone().unwrap();
                write!(user_stream, "{}", message).unwrap();
                user_stream.flush().unwrap();
            }
            break;
        }

      
        let message = format!("{}: {}\n", username, message);
        println!("{}", message);
        for user in users.lock().unwrap().iter() {
            if user != &username {
                let mut user_stream = stream.get_ref().try_clone().unwrap();
                write!(user_stream, "{}", message).unwrap();
                user_stream.flush().unwrap();
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("localhost:5000").unwrap();
    let users = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let users = users.clone();
                thread::spawn(move || {
                    handle_client(stream, users);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
