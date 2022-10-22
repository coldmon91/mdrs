
use std::{fs, io::Read};
use std::fs::File;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt}, 
    net::{TcpListener, TcpStream}, sync::broadcast::{self, Sender, Receiver}
};

use log::{info, debug, warn, error};

const BUFFSIZE : usize = 16384;

pub async fn file_handler(mut socket : TcpStream, tx : Sender<Vec<u8>>, mut rx : Receiver<Vec<u8>>) {
    let (mut read_sock, mut write_sock) = socket.split();
    let mut buffer = [0u8; BUFFSIZE];

    let file_path = "sample_videos/LT.mov";
    match File::open(&file_path) {
        Ok(mut file) => {
            let metadata = fs::metadata(&file_path).expect("unable to read metadata");
            let mut tmp_buff = vec![0; metadata.len() as usize];
            match file.read(&mut tmp_buff) {
                Ok(size) => {
                    info!("read file {} bytes", size);
                    socket.write_all(&tmp_buff).await.unwrap();
                }
                Err(e) => {
                    error!("error reading file: {}", e);
                }
            }
        },
        Err(e) => {
            error!("file open error: {}", e);
            return;
        }
    };
}

