use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::console::ask_for_input;

pub async fn run(address: String) -> std::io::Result<()>{
    println!("{address}");
    let mut stream = TcpStream::connect(format!("{}", address)).await?;
    loop {
        let s = ask_for_input("Write your message: ");
        if s == "exit"{
            break;
        }
        stream.write_all(s.as_bytes()).await.unwrap();
    }

    Ok(())
}