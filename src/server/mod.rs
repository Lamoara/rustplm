use std::error::Error;

use tokio::{io::{self}, net::{TcpListener, TcpStream}};

use crate::console::{clear_console, show_menu};

pub async fn run() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Listening at: {}", listener.local_addr().unwrap());


    let listener_handle = tokio::spawn(async move {
        handle_socket_inputs(listener).await;
    });

    let menu_handle = tokio::spawn(async move {
        loop {
            let option = show_menu(vec!["Show processes", "Stop"]);
            match option {
                1 => {
                    clear_console();
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

async fn handle_client(listener: TcpStream) -> Result<(), Box<dyn Error>> {
    println!("Client connected");
    
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
