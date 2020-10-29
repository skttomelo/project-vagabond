use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use std::sync::{Arc, RwLock};
use std::thread;

mod game_data;
mod geometry;
mod gui_data;
mod entity_data;
mod server_data;

use server_data::ServerGameMatch;

fn main() {
    // create initial match struct and id counter
    let game_match = Arc::new(RwLock::new(ServerGameMatch::new()));
    let id_counter = Arc::new(RwLock::new(0usize));

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
                thread::spawn(move || {
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

fn handle_client(
    mut socket: TcpStream,
    game_match: Arc<RwLock<ServerGameMatch>>,
    counter: Arc<RwLock<usize>>,
) {
    let mut data = [0u8; 1024];

    let mut string_data: String;
    let id: usize;

    // check if the server already has two people on it
    // shut the stream down and return nothing if true
    {
        let c = counter.write().unwrap();
        if *c == 2 {
            println!(
                "Closing connection with {} because server is handling max amount of clients",
                socket.peer_addr().unwrap()
            );
            socket.shutdown(Shutdown::Both).unwrap();
            return;
        }
    }

    // increment counter by 1
    {
        let mut c = counter.write().unwrap();
        *c += 1;
        id = *c - 1;
    }

    socket
        .write_all(&id.to_string().as_bytes())
        .expect("Unable to write id value to stream");
    socket.flush().expect("Unable to flush stream");

    // establish connection loop
    while match socket.read(&mut data) {
        Ok(_) => true,
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                socket.peer_addr().unwrap()
            );
            socket.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {
        // read the data in from the socket and write it to a String
        string_data = String::from(from_utf8(&data).unwrap());
        string_data = string_data.trim_matches(char::from(0)).to_owned();

        // Deserialize the json data in the String to a ServerGameMatch struct
        let match_details: ServerGameMatch = serde_json::from_str(&string_data).unwrap();

        // update the player's data on the server
        game_match
            .write()
            .unwrap()
            .update_entity(0, match_details.server_entities[0].clone());
        game_match
            .write()
            .unwrap()
            .update_entity(1, match_details.server_entities[1].clone());

        // Serialize the data on server into json and then send it back to the client
        let match_details_json = serde_json::to_string(&(*game_match.write().unwrap()))
            .expect("Could not serialize game match");

        socket
            .write_all(&match_details_json.as_bytes())
            .expect("Could not write data to stream");
        socket.flush().expect("Could not flush stream");
    }

    // decrement counter by 1
    {
        let mut c = counter.write().unwrap();
        if *c > 0 {
            *c -= 1;
        }
    }

    socket.shutdown(Shutdown::Both).unwrap();
}
