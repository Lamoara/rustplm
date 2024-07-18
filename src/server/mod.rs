use std::{collections::HashMap, error::Error, fs, net::{IpAddr, SocketAddr}, sync::Arc};
use serde::{Deserialize, Serialize};
use tokio::{io::{self, AsyncWriteExt}, net::{TcpListener, TcpStream}, sync::Mutex};
use crate::console::{clear_console, show_menu};

type ConnHash = Arc<Mutex<HashMap<ClientIP, ClientState>>>;
type ClientIP = IpAddr;
type ClientState = String;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserDatabase {
    users: HashMap<String, User>,
}

impl UserDatabase {
    fn load_from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let data = fs::read_to_string(file_path)?;
        let db: UserDatabase = serde_json::from_str(&data)?;
        Ok(db)
    }

    fn save_to_file(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let data = serde_json::to_string_pretty(&self)?;
        fs::write(file_path, data)?;
        Ok(())
    }
}


pub async fn run() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Listening at: {}", listener.local_addr()?);

    let connections: ConnHash  = Arc::new(Mutex::new(HashMap::new()));

    let connections_clone = connections.clone();
    let listener_handle = tokio::spawn(async move {
        handle_socket_inputs(listener, connections_clone).await;
    });

    let menu_handle: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        loop {
            let option: usize = show_menu(vec!["Show processes", "Stop"]);
            match option {
                1 => {
                    clear_console();
                    println!("{:?}", connections.lock().await);
                    continue
                },
                2 => break,
                _ => continue,
            }
        }
    });

    tokio::select! {
        _ = listener_handle => {},
        _ = menu_handle => {},
    }

    Ok(())
}


async fn handle_socket_inputs(listener: TcpListener, connections: ConnHash) {
    loop {
        match listener.accept().await {
            Ok((socket, dir)) => {
                let connections_clone = connections.clone();
                tokio::spawn(async move{
                    check_new_conn(connections_clone, dir).await;
                    handle_client(socket).await.unwrap();
                });
            }
            Err(e) => eprintln!("Failed to accept connection: {}", e),
        }
    }
}


async fn check_new_conn(connections: ConnHash, dir: SocketAddr){
    let mut conn = connections.lock().await;
    if !conn.contains_key(&dir.ip()){
        conn.insert(dir.ip(), "New".to_string());
    }
    drop(conn);
}


async fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    loop {
        // Wait for the socket to be readable
        stream.readable().await?;

        let mut buf: Vec<u8> = Vec::with_capacity(4096);

        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                let str = core::str::from_utf8(&buf)?;
                handle_client_input(str, &mut stream).await
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
}


async fn handle_client_input(command: &str, stream: &mut TcpStream){
    //Get command type
    let args: Vec<&str> = command.split("/").collect();
    match args[0] {
        "login" => handle_login(args, stream).await,
        "signup" => handle_signup(args, stream).await,
        _ => return 
    }
}


async fn handle_login(args: Vec<&str>, stream: &mut TcpStream){
    if args.len() < 3 {
        stream.write_all("Invalid command\n".as_bytes()).await.unwrap();
        return;
    }
    let username = args[1];
    let password = args[2];


    let db = UserDatabase::load_from_file("users").unwrap_or(UserDatabase { users: HashMap::new() });

    if let Some(user) = db.users.get(username) {
        if user.password == password {
            stream.write_all(b"Login successful\n").await.unwrap();
        } else {
            stream.write_all(b"Invalid username or password\n").await.unwrap();
        }
    } else {
        stream.write_all(b"Invalid username or password\n").await.unwrap();
    }
}


async fn handle_signup(args: Vec<&str>, stream: &mut TcpStream) {
    if args.len() < 3 {
        stream.write_all(b"Invalid command\n").await.unwrap();
        return;
    }
    let username = args[1];
    let password = args[2];

    let mut db = UserDatabase::load_from_file("users").unwrap_or(UserDatabase { users: HashMap::new() });

    if db.users.contains_key(username) {
        stream.write_all(b"Username already exists\n").await.unwrap();
    } else {
        db.users.insert(username.to_string(), User { username: username.to_string(), password: password.to_string() });
        db.save_to_file("users").unwrap();
        stream.write_all(b"Signup successful\n").await.unwrap();
    }
}