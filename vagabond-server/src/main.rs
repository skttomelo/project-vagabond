use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use serde_json;
use serde::{Serialize, Deserialize};

mod entities;
use entities::Player;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 1024]; // using 50 byte buffer
    // let mut writer = BufWriter::new(&stream);
    // let mut reader = BufReader::new(&stream);
    // reader.read_exact((&mut response).unwrap();
    // println!("{}",response);

    // let json = std::str::from_utf8(&data[..]).expect("could not read");
    let mut deserialize_me = serde_json::Deserializer::from_reader(&stream);
    let client: Player = Player::deserialize(&mut deserialize_me).unwrap();
    // let client: Player = serde_json::from_str(&json).expect("could not deserialize data");
    println!("{:?}", client);

    while match stream.read(&mut data) {
        Ok(_size) => {
            // echo everything!
            // let mut reader = BufReader::new(&stream);
            // let mut response = String::from("");
            // println!("{}", reader.read_to_string(&mut response));

            // stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {
        
        break;
    }

    stream.shutdown(Shutdown::Both).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 1337");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}