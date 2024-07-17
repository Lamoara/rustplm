use tokio::{net::{TcpListener, TcpStream}, stream};

pub async  fn run() -> std::io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    println!("Listening at: {}", listener.local_addr().unwrap().port());


    loop {
        let (socket, _) = listener.accept().await?;
        handle_client(socket).await;
    }
    Ok(())
}
async fn handle_client(stream: TcpStream)
{

}
