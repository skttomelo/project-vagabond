use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use serde_json;

mod entities;
use entities::{Point2, Player};

// I want the client to first receive an id from the server once it connects, then I want the main communication "loop" to occur

fn main() {
    let mut id = 0;

    let data_to_send = Player::new(
        Point2::<f32>{x: 50.0,y: 50.0},
        Point2::<f32>{x: 0.0, y: 0.0},
        30.0);
    // convert data into json string
    let data_as_json = serde_json::to_string(&data_to_send).expect("could not serialize data.");

    while match TcpStream::connect("127.0.0.1:1337") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 1337");
            
            // convert data into json string
            let data_as_json = serde_json::to_string(&data_to_send).expect("could not serialize data.");
            // get json string as a vector of bytes
            let msg = &data_as_json.into_bytes();

            // send json string to server
            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            // let mut data = [0 as u8; 6]; // using 6 byte buffer
            // match stream.read_exact(&mut data) {
            //     Ok(_) => {
            //         if &data == msg {
            //             println!("Reply is ok!");
            //         } else {
            //             let text = from_utf8(&data).unwrap();
            //             println!("Unexpected reply: {}", text);
            //         }
            //     },
            //     Err(e) => {
            //         println!("Failed to receive data: {}", e);
            //     }
            // }
            true
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
            false
        }
    } {

    }
    println!("Terminated.");
}

fn handle_connection(stream: TcpStream) {

}