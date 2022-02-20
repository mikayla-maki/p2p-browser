mod datastore;

use std::net::SocketAddr;
use tokio::io;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use crate::datastore::Datastore;
use crate::datastore::FileSystemDriver;

#[tokio::main]
async fn main() -> io::Result<()> {
    let datastore = FileSystemDriver::init();
    let listener = TcpListener::bind("localhost:8492").await?;

    loop {
        match listener.accept().await {
            Ok((socket, remote)) => {
                println!("Connection from: {:?}", remote);
                tokio::spawn(async move {
                    process(socket, remote, datastore.clone());
                });
            }
            Err(e) => {
                println!("Couldn't connect: {:?}", e);
            }
        }
    }
}



fn process(socket: TcpStream, remote: SocketAddr, datastore: impl Datastore) {
    println!("Processing: {:?}", remote);
}