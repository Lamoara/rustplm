use std::{collections::HashMap, io::BufRead, sync::Arc};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, runtime::Runtime, stream, sync::Mutex};

use crate::console::{clear_console, show_menu};

pub async fn run() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening at: {}", listener.local_addr().unwrap().port());


    let listener_handle = tokio::spawn(async move {
        handle_socket_inputs(listener).await;
    });

    let menu_handle = tokio::spawn(async move {
        loop {
            clear_console();
            let option = show_menu(vec!["Show processes", "Stop"]);
            match option {
                1 => continue,
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


async fn handle_socket_inputs(listener: TcpListener) {
    loop {
        match listener.accept().await {
            Ok((socket, _)) => {
                handle_client(socket).await;
            }
            Err(e) => eprintln!("Failed to accept connection: {}", e),
        }
    }
}

async fn handle_client(listener: TcpStream) {
    println!("Client connected");
}
