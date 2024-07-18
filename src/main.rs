use std::{env, process::exit};

pub mod server;
pub mod client;
pub mod console;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        eprintln!("Especify client or server: cargo run -- client/server");
        exit(1);
    }

    match args[1].as_str(){
        "client"|"c" => {client::run(args[2].to_string()).await.unwrap()},
        "server"|"s" => {server::run().await.unwrap()},
        _ => {        
            eprintln!("Especify client or server: cargo run -- client/server");
            exit(1);}
    }
}
