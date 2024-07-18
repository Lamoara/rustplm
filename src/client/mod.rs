use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::console::{ask_for_input, show_menu};

pub async fn run(address: String) -> std::io::Result<()>{
    println!("{address}");
    let mut stream = TcpStream::connect(format!("{}", address)).await?;

    loop{
        let option: usize = show_menu(vec!["Login", "Sign Up", "Exit"]);
        match option {
            1 => continue,
            2 => continue,
            3 => {
                stream.shutdown().await?;
                break
            },
            _ => continue
        }
    }

    Ok(())
}