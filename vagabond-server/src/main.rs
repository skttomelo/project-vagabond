// use serde_json;
// use serde::{Serialize, Deserialize};

use tokio::net::{TcpListener, TcpStream};

// use std::io;

mod entities;
use entities::Entity;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1::1337";
    let mut listener = TcpListener::bind(addr).await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        
        tokio::spawn(async move {
            process_message(socket).await;
        });
    }
}

async fn process_message(mut socket: TcpStream) {
    let (mut reader, mut writer) = socket.split();

    let ent: Entity = Entity::from_json(&reader); 

    // player.update();

    

}