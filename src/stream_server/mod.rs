mod file_handler;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt}, 
    net::{TcpListener, TcpStream}, sync::broadcast::{self, Sender, Receiver}
};

use log::{info, debug, warn, error};

const BUFFSIZE : usize = 16384;

enum ReqType {
    File,
    Stream,
}

fn parse_message(req : &str) -> Option<ReqType> {
    return Some(ReqType::File);
}

pub async fn handler(mut socket : TcpStream, tx : Sender<Vec<u8>>, mut rx : Receiver<Vec<u8>>) {
    let (mut read_sock, mut write_sock) = socket.split();
    let mut buffer = [0u8; BUFFSIZE];
    let result = read_sock.read(&mut buffer).await;
    match result {
        Ok(size) => {
            let req = String::from_utf8_lossy(&buffer[..size]);
            debug!("request: {}", req);
            match parse_message(&req) {
                Some(ReqType::File) => {
                    file_handler::file_handler(socket, tx, rx).await;
                }
                Some(ReqType::Stream) => {
                    // stream_sender(socket, tx, rx).await;
                }
                None => {
                    error!("invalid request");
                }
            }
        }
        Err(e) => {
            error!("error reading from socket: {}", e);
        }
    }
}

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
            handler(socket, tx, rx).await;
            info!("out client {}", &ip);
        });
    }
}