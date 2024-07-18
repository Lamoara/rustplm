use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use std::io::Result;
use crate::console::{ask_for_input, show_menu};

pub async fn run(address: String) -> Result<()> {
    loop {
        let mut stream = TcpStream::connect(format!("{}", address)).await?;

        let option: usize = show_menu(vec!["Login", "Sign Up", "Exit"]);
        match option {
            1 => login(&mut stream).await?,
            2 => sign_up(&mut stream).await?,
            3 => {
                stream.shutdown().await?;
                break;
            },
            _ => continue,
        }

        stream.shutdown().await?;
    }

    Ok(())
}


async fn login(stream: &mut TcpStream) -> Result<()> {
    let username: String = ask_for_input("Introduce your username: ");
    let password: String = ask_for_input("Introduce your password: ");
    send_login(username, password, stream).await
}


async fn sign_up(stream: &mut TcpStream) -> Result<()> {
    let username: String = ask_for_input("Introduce your username: ");
    let password: String = ask_for_input("Introduce your password: ");
    if ask_for_input("Repeat your password: ") != password {
        println!();
        println!("Passwords don't match");
        return Ok(());
    }
    send_sign_up(username, password, stream).await
}


async fn send_login(username: String, password: String, stream: &mut TcpStream) -> Result<()> {
    stream.write_all(format!("login/{}/{}", username.trim_end(), password.trim_end()).as_bytes()).await?;

    let mut buf = vec![0; 4096];
    let n = stream.read(&mut buf).await?;
    let response = std::str::from_utf8(&buf[..n]).unwrap_or("Invalid UTF-8 sequence");
    println!("{}", response);

    Ok(())
}


async fn send_sign_up(username: String, password: String, stream: &mut TcpStream) -> Result<()> {
    stream.write_all(format!("signup/{}/{}", username.trim_end(), password.trim_end()).as_bytes()).await?;

    let mut buf = vec![0; 4096];
    let n = stream.read(&mut buf).await?;
    let response = std::str::from_utf8(&buf[..n]).unwrap_or("Invalid UTF-8 sequence");
    println!("{}", response);

    Ok(())
}
