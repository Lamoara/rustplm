use tokio::net::TcpStream;

pub async fn run(address: String) -> std::io::Result<()>{
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", address)).await?;

    Ok(())
}