use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::str::from_utf8;

mod game_data;
use game_data::{GameMatch, GameMatchServer, Point2};

// I want the client to first receive an id from the server once it connects, then I want the main communication "loop" to occur

fn main() {
    match TcpStream::connect("127.0.0.1:1337") {
        Ok(stream) => {
            println!("Successfully connected to server in port 1337");

            handle_connection(stream, Option::None)
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }

    println!("Terminated.");
}

fn handle_connection(mut stream: TcpStream, game_match: Option<GameMatch>) {
    let mut data = [0u8; 4096];

    let id: u8;
    let mut string_data: String;

    // first acquire id
    stream.read(&mut data[..]).unwrap();
    string_data = String::from(from_utf8(&data).unwrap());
    string_data = String::from(string_data.trim_matches(char::from(0)));
    id = string_data.parse().unwrap(); // because id's type is declared earlier we do not need to do `parse::<u8>()`

    println!("we have made it here pog id:{}", id);

    // create GameMatch if one doesn't exist already
    let mut game_match = match game_match {
        Some(mut gmatch) => {
            gmatch.id = id;
            gmatch
        }
        _ => GameMatch::new(),
    };

    // because this is the basic client we will artificially update game match data
    game_match.entities[(id as usize) - 1].set_vel(Point2::<f32> { x: 1.0, y: 0.0 });
    for entity in &mut game_match.entities {
        entity.update();
    }
    // serialize data to send to the server
    let serialized_data =
        serde_json::to_string(&game_match).expect("Couldn't serialize game match");

    // convert data to byte array and send to server
    let data_as_bytes = serialized_data.into_bytes();
    stream
        .write_all(&data_as_bytes)
        .expect("Could not write bytes to stream");
    stream.flush().expect("Could not flush stream");

    // we will then intercept the new data and deserialize it
    stream.read(&mut data[..]).unwrap();
    string_data = String::from(from_utf8(&data).unwrap());
    string_data = String::from(string_data.trim_matches(char::from(0)));
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
