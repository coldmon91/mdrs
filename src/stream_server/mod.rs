mod file_handler;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt}, 
    net::{TcpListener, TcpStream}, sync::broadcast::{self, Sender, Receiver}
};

use log::{info, debug, warn, error};

pub async fn run(addr : &str) {
    info!("server run {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    let (_tx, _rx) = broadcast::channel::<Vec<u8>>(10);

    loop {  
        let (socket , _addr ) = listener.accept().await.unwrap();
        let tx = _tx.clone();
        let rx = tx.subscribe();

        tokio::spawn(async move {
            let ip = socket.peer_addr().unwrap().ip().to_string();
            info!("new client {}", &ip);
            file_handler::file_sender(socket, tx, rx).await;
            info!("out client {}", &ip);
        });
    }
}