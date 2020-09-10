use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};
// use std::str::from_utf8;
use serde_json;
// use serde::{Serialize, Deserialize};

mod game_data;
use game_data::{Point2, GameMatch, GameMatchServer};

// I want the client to first receive an id from the server once it connects, then I want the main communication "loop" to occur

fn main() {
    // let data_to_send = Player::new(
    //     Point2::<f32>{x: 50.0,y: 50.0},
    //     Point2::<f32>{x: 0.0, y: 0.0},
    //     30.0);
    // convert data into json string
    // let data_as_json = serde_json::to_string(&data_to_send).expect("could not serialize data.");

    match TcpStream::connect("127.0.0.1:1337") {
        Ok(stream) => {
            println!("Successfully connected to server in port 1337");

            handle_connection(stream, Option::None);
            
            // convert data into json string
            // let data_as_json = serde_json::to_string(&data_to_send).expect("could not serialize data.");
            // get json string as a vector of bytes
            // let msg = &data_as_json.into_bytes();

            // send json string to server
            // stream.write(msg).unwrap();
            // println!("Sent Hello, awaiting reply...");

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
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }

    println!("Terminated.");
}

fn handle_connection(mut stream: TcpStream, game_match: Option<GameMatch>) {
    // let mut data = [0u8; 4096];

    let id: u8;
    let mut string_data = String::new();

    // first acquire id
    stream.read_to_string(&mut string_data).unwrap();
    id = string_data.parse().unwrap(); // because id's type is declared earlier we do not need to do `parse::<u8>()`

    // create GameMatch if one doesn't exist already
    let mut game_match = match game_match {
        Some(mut gmatch) => {
            gmatch.id = id;
            gmatch
        },
        _ => GameMatch::new()
    };
    
    // because this is the basic client we will artificially update game match data
    game_match.entities[id as usize].set_vel(Point2::<f32>{x: 1.0, y: 0.0});
    for entity in &mut game_match.entities {
        entity.update();
    }
    // serialize data to send to the server
    let serialized_data = serde_json::to_string(&game_match).expect("Couldn't serialize game match");

    // convert data to byte array and send to server
    let data_as_bytes = serialized_data.into_bytes();
    stream.write_all(&data_as_bytes).expect("Could not write bytes to stream");
    stream.flush().expect("Could not flush stream");

    // we will then intercept the new data and deserialize it
    stream.read_to_string(&mut string_data).expect("Could not read data to string from stream");
    let server_match: GameMatchServer = serde_json::from_str(string_data.as_str()).unwrap();
    let mut server_match_iter = server_match.entities.into_iter();
    // update the game match
    for entity in &mut game_match.entities {
        entity.update_data(0u8, server_match_iter.next().unwrap());
    }

    // output data to console to show the game match after data is received from the server
    println!("{:?}", game_match);

    // close the stream when we are done    
    stream.shutdown(Shutdown::Both).unwrap();
}