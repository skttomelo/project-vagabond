use serde_json;
// use serde::{Serialize, Deserialize};

use std::thread;
use std::io::{Read, Write};
use std::sync::{Arc, RwLock};
use std::net::{TcpListener, TcpStream, Shutdown};


mod game_data;
use game_data::GameMatch;

fn main() {

    // create initial match struct and id counter
    let game_match = Arc::new(RwLock::new(GameMatch::new()));
    let id_counter = Arc::new(RwLock::new(0u8));

    // clone so we can move into closure
    let game_match_inner = game_match.clone();

    // bind ip address to server listener
    let addr = "127.0.0.1:1337";
    let listener = TcpListener::bind(addr).unwrap();

    // handle each connection to server
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());

                let game_match = game_match_inner.clone();
                let id_counter_inner = id_counter.clone();

                // spawn thread so we can accept more connections
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream, game_match, id_counter_inner)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
}

fn handle_client(mut socket: TcpStream, game_match: Arc<RwLock<GameMatch>>, counter: Arc<RwLock<u8>>) {
    let mut data = [0u8; 4096];

    let mut string_data = String::new();
    let id: u8;
    // increment counter by 1
    {
        let mut c = counter.write().unwrap();
        *c += 1;
        id = *c;
    }

    socket.write_all(&[id]).expect("Unable to write id value to stream");
    socket.flush().expect("Unable to flush stream");


    // establish connection loop
    while match socket.read(&mut data) {
        Ok(_) => true,
        Err(_) => {
            println!("An error occurred, terminating connection with {}", socket.peer_addr().unwrap());
            socket.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {
        // read the data in from the socket and write it to a String
        socket.read_to_string(&mut string_data).unwrap();

        // Deserialize the json data in the String to a Match struct
        let match_details: GameMatch = serde_json::from_str(string_data.as_str()).unwrap();

        // update the player's data on the server
        game_match.write().unwrap().update_entity(id, match_details.entities[id as usize].clone());

        // Serialize the data on server into json and then send it back to the client
        let match_details_json = serde_json::to_string(&(*game_match.write().unwrap())).expect("Could not serialize game match");

        socket.write_all(&match_details_json.as_bytes()).expect("Could not write data to stream");
    }

    socket.shutdown(Shutdown::Both).unwrap();
}