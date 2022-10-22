

use tokio ::{
    io::{AsyncReadExt, AsyncWriteExt}, 
};

use std::{fs, io::{Read, Write}};
use std::fs::File;

#[tokio::main]
async fn main() {

    let addr = "127.0.0.1:8080";
    let mut stream = tokio::net::TcpStream::connect(addr).await.unwrap();
    println!("stream: {:?}", stream);

    let msg = "LT.mov";
    stream.write_all(msg.as_bytes()).await.unwrap();

    let mut f = File::create("received.mov").expect("unable to create file");
    loop {
        let mut buffer = vec![0; 65535];
        println!("wait for data");
        let read_size = stream.read(&mut buffer).await.unwrap();
        match read_size {
            0 => {
                println!("connection closed");
                break;
            }
            _ => {
                println!("read {} bytes", read_size);
                f.write_all(&buffer).expect("unable to write file");
            }
        }
    }
}