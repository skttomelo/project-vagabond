use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;
use std::env;
use std::time::Duration;

use bincode;

mod geometry;
mod animate;
mod server_data;

use animate::Animator;
use server_data::ServerGameMatch;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

pub struct Worker; // will implement later

fn main() {
    // command line args
    let args: Vec<String> = env::args().collect();
    let ip_address = match args.len() {
        2 => args[1].clone(),
        _ => String::from("127.0.0.1:1337"),
    };

    // initialize ThreadPool
    let mut thread_pool = ThreadPool{threads: Vec::new()};

    // create initial match struct and id counter
    let game_match = Arc::new(RwLock::new(ServerGameMatch::new()));
    // let id_counter = Arc::new(RwLock::new(0usize));
    let clock_timer = Arc::new(RwLock::new(Animator::new(60, Duration::from_secs(1))));

    // clone so we can move into closure
    let game_match_inner = game_match.clone();

    // bind ip address to server listener
    let addr = ip_address;
    let listener = TcpListener::bind(addr).unwrap();

    // handle each connection to server
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());

                let game_match = game_match_inner.clone();
                // let id_counter_inner = id_counter.clone();
                let clock_timer_inner = clock_timer.clone();
                
                let connection_id = thread_pool.threads.len();
                println!("{}", &connection_id);
                if connection_id == 2 {
                    stream.shutdown(Shutdown::Both).unwrap();
                } else {
                    // spawn thread so we can accept more connections
                    thread_pool.threads.push(thread::spawn(move || {
                        // connection succeeded
                        handle_client(stream, game_match, clock_timer_inner, connection_id)
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
    clock_timer: Arc<RwLock<Animator>>,
    id: usize
) {
    let mut data = [0u8; 1024];

    socket
        .write_all(&id.to_string().as_bytes())
        .expect("Unable to write id value to stream");
    socket.flush().expect("Unable to flush stream");

    if id == 1 {
        clock_timer.write().unwrap().paused = false;
    }

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
        clock_timer.write().unwrap().update();
        
        let current_time = clock_timer.read().unwrap().current_frame();
        game_match.write().unwrap().update_clock(current_time);

        game_match
            .write()
            .unwrap()
            .update_entity(id, match_details.server_entities[id].clone());

        // Serialize the data on server and then send it back to the client
        let serialized_data: Vec<u8> = bincode::serialize(&(*game_match.write().unwrap())).unwrap();
        socket.write(&serialized_data).unwrap();

        socket.flush().expect("Could not flush stream");
    }

    socket.shutdown(Shutdown::Both).unwrap();
}
