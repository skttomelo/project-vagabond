// based closely off repo found here: https://github.com/jaybutera/mmo-rust-server/

use tokio::runtime;
use tokio::runtime::TaskExecutor;

use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::{Arc, RwLock};

use websocket::message::OwnedMessage;
use websocket::server::InvalidConnection;
use websocket::server::r#async::Server;

use futures::{Future,Stream,Sink};
use futures::future::{self, Loop};

mod entities;
use entities::{Point2, Entity};

fn main() {
    let runtime = runtime::Builder::new().build().unwrap();
    let executor = runtime.executor();
    let server = Server::bind("127.0.0.1:1337", &runtime.reactor()).expect("Failed to create the server");

    // use hashmap to store sink value with id key
    // sink is used to send data to the client
    let connections = Arc::new(RwLock::new(HashMap::new()));
    // hashmap id:entity pairs
    let entities = Arc::new(RwLock::new(HashMap::new()));
    // will be used to assign a unique id to each new player that connects to the server
    let counter = Arc::new(RwLock::new(0));

    // clone references to these states so they can be moved into the connection handler closure
    let connections_inner = connections.clone();
    let entities_inner    = entities.clone();
    let executor_inner    = executor.clone();

    // we spawn a future for every new connection request from a client
    let connection_handler = server.incoming()
        .map_err(|InvalidConnection { error, .. }| error)
        .for_each(move |(upgrade, addr)| {
            // we clone so we can move into next closure "f"
            let connections_inner = connections_inner.clone();
            let entities = entities_inner.clone();
            let counter_inner = counter.clone();
            let executor_2inner = executor_inner.clone();

            // this completes the connection and then processes the sink and stream
            let accept = upgrade.accept().and_then(move |(framed,_)| {
                let (sink, stream) = framed.split();

                // we now increment the counter in closure by locking the rwlock
                {
                    let mut c = counter_inner.write().unwrap();
                    *c += 1;
                }

                // now we assign an id to the new connection and associate it with a new player and the sink
                let id = *counter_inner.read().unwrap();
                connections_inner.write().unwrap().insert(id,sink);
                entities.write().unwrap().insert(id, Entity{id: id, pos: Point2{x:0.0,y:0.0}, vel: Point2{x:0.0,y:0.0}, size: 0.0});
                let c = *counter_inner.read().unwrap();

                // closure "f"
                // spawns stream that will process future messages from client
                let f = stream.for_each(move |msg| {
                    process_message(c, &msg, entities.clone());
                    Ok(())
                }).map_err(|_| ());

                executor_2inner.spawn(f);

                Ok(())
            }).map_err(|_| ());

            executor_inner.spawn(accept);
            Ok(())
        })
        .map_err(|_| ());

        // game loop
    let send_handler = future::loop_fn((), move |_| {
        // we clone once again because of "and_then" later on takes a FnMut closure
        let connections_inner = connections.clone();
        let executor = executor.clone();
        let entities_inner = entities.clone();

        // we create a delay that will make the loop run ~64 times a second
        tokio::timer::Delay::new(Instant::now() + Duration::from_millis(16))
            .map_err(|_| ())
            .and_then(move |_| {
                let mut conn = connections_inner.write().unwrap();
                let ids = conn.iter().map(|(k,v)| {k.clone()}).collect::<Vec<_>>(); // we iterate across all connections and clone the ids, then we collect them into a vector

                for id in ids.iter() {
                    // we have to take ownership of the sink in order to send data on it
                    // because of this, the only way we can take ownership of the hashmap val is to remove it
                    // then later we will puyt it back together
                    let sink = conn.remove(id).unwrap();

                    // serialize entity into json
                    // NOTE: come back to this later and change it to use serde
                    let entities = entities_inner.read().unwrap();
                    let first = match entities.iter().take(1).next() {
                        Some((_,e)) => e,
                        None => continue
                    };
                    let serial_entities = format!("[{}]", entities.iter().skip(1)
                        .map(|(_,e)| e.to_json())
                        .fold(first.to_json(), |acc,s| format!("{},{}",s,acc)));

                    
                    // clone for future "f"
                    let connections = connections_inner.clone();
                    let id = id.clone();

                    // we send game state to client
                    let f = sink.send(OwnedMessage::Text(serial_entities))
                        .and_then(move |sink| {
                            // re-insert the entry to the connections map
                            connections.write().unwrap().insert(id.clone(), sink);
                            Ok(())
                        }).map_err(|_| ());

                    executor.spawn(f);
                }

                match true {
                    true => Ok(Loop::Continue(())),
                    false => Ok(Loop::Break(()))
                }
            })
    });

    // block the main thread to wait for the connection_handler and sender_handler streams to finish
    // (should never occur unless there is an error)
    runtime.block_on_all(connection_handler.select(send_handler))
        .map_err(|_| println!("Error while running core loop"))
        .unwrap();
}

// nitty gritty
// this is where we update entity states
fn process_message(id: u32, msg: &OwnedMessage, entities: Arc<RwLock<HashMap<u32,Entity>>>) {
    println!("")
}