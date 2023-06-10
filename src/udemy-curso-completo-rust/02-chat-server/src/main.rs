use std::net::SocketAddr;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{tcp::ReadHalf, TcpListener},
    sync::broadcast::{channel, Receiver, Sender},
};

#[tokio::main]
async fn main() {
    let listener: TcpListener = TcpListener::bind("localhost:8080").await.unwrap();
    println!("listener: {:#?}", listener);

    let (tx, _rx) = channel::<(String, SocketAddr)>(10);
    println!("tx: {:#?}", tx);
    println!("_rx: {:#?}", _rx);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("socket: {:#?}", socket);
        println!("addr: {:#?}", addr);

        let tx: Sender<(String, SocketAddr)> = tx.clone();
        println!("tx: {:#?}", tx);
        let mut rx: Receiver<(String, SocketAddr)> = tx.subscribe();
        println!("rx: {:#?}", rx);

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            println!("reader: {:#?}", reader);
            println!("writer: {:#?}", writer);
            let mut reader: BufReader<ReadHalf> = BufReader::new(reader);
            let mut line: String = String::new();

            loop {
                tokio::select! {
                    bytes_read = reader.read_line(&mut line) => {
                        println!("Recived: {:#?}", bytes_read);
                        tx.send((line.clone(), addr)).unwrap();
                        println!("tx: {:#?}", tx);
                        line.clear();
                    }
                    received = rx.recv() => {
                        let (message, message_addr) = received.unwrap();
                        println!("message_addr: {:#?}", message_addr);
                        println!("message: {:#?}", message);
                        if message_addr != addr {
                            writer.write_all(message.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
