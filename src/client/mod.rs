use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::console::{ask_for_input, show_menu};

pub async fn run(address: String) -> std::io::Result<()>{
    let mut stream = TcpStream::connect(format!("{}", address)).await?;

    loop{
        let option: usize = show_menu(vec!["Login", "Sign Up", "Exit"]);
        match option {
            1 => {
                let username: String = ask_for_input("Introduce your username: ");
                let password: String = ask_for_input("Introduce your password: ");
                send_login(username, password, &mut stream).await?
            },
            2 => {
                let username: String = ask_for_input("Introduce your username: ");
                let password: String = ask_for_input("Introduce your password: ");
                if ask_for_input("Repeat your password: ") != password{
                    println!();
                    println!("Passwords dont match");
                    continue;
                }
                send_sign_up(username, password, &mut stream).await?
            },
            3 => {
                stream.shutdown().await?;
                break
            },
            _ => continue
        }
    }

    Ok(())
}

async fn send_login(username: String, password: String, stream: &mut TcpStream) -> std::io::Result<()>{
    //let login = Login::new(username, password);
    stream.write_all("Test".as_bytes()).await
}

async fn send_sign_up(username: String, password: String, stream: &mut TcpStream) -> std::io::Result<()>{
    stream.write_all(format!("signup {username} {password}").as_bytes()).await

}