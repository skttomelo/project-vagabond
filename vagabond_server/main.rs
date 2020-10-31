use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;

use bincode;

mod geometry;
mod server_data;

use server_data::ServerGameMatch;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

pub struct Worker; // will implement later

fn main() {
    // initialize ThreadPool
    let mut thread_pool = ThreadPool{threads: Vec::new()};

    // create initial match struct and id counter
    let game_match = Arc::new(RwLock::new(ServerGameMatch::new()));
    // let id_counter = Arc::new(RwLock::new(0usize));

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
                // let id_counter_inner = id_counter.clone();
                
                let connection_id = thread_pool.threads.len();
                println!("{}", &connection_id);
                if connection_id == 2 {
                    stream.shutdown(Shutdown::Both).unwrap();
                } else {
                    // spawn thread so we can accept more connections
                    thread_pool.threads.push(thread::spawn(move || {
                        // connection succeeded
                        handle_client(stream, game_match, connection_id)
                    }));
                }
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
    id: usize
) {
    let mut data = [0u8; 1024];

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
        // Deserialize the data to a ServerGameMatch struct
        let match_details: ServerGameMatch = bincode::deserialize(&data).unwrap();

        // update the player's data on the server
        game_match
            .write()
            .unwrap()
            .update_entity(0, match_details.server_entities[0].clone());
        game_match
            .write()
            .unwrap()
            .update_entity(1, match_details.server_entities[1].clone());

        // Serialize the data on server and then send it back to the client
        let serialized_data: Vec<u8> = bincode::serialize(&(*game_match.write().unwrap())).unwrap();
        socket.write(&serialized_data).unwrap();

        socket.flush().expect("Could not flush stream");
    }

    socket.shutdown(Shutdown::Both).unwrap();
}
