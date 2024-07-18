use std::{collections::HashMap, error::Error, net::{IpAddr, SocketAddr}, sync::Arc};

use tokio::{io::{self}, net::{TcpListener, TcpStream}, sync::Mutex};

use crate::console::{clear_console, show_menu};

type ConnHash = Arc<Mutex<HashMap<ClientIP, ClientState>>>;
type ClientIP = IpAddr;
type ClientState = String;

pub async fn run() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Listening at: {}", listener.local_addr().unwrap());

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
                check_new_conn(connections.clone(), dir).await;
                handle_client(socket).await.unwrap();
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

async fn handle_client(listener: TcpStream) -> Result<(), Box<dyn Error>> {
    loop {
        // Wait for the socket to be readable
        listener.readable().await?;

        let mut buf: Vec<u8> = Vec::with_capacity(4096);

        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match listener.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                let str = core::str::from_utf8(&buf).unwrap();
                println!("{}", str); // Prints "⚠"
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
