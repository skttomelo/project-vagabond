use std::str;
use std::clone::Clone;

use serde::{Serialize, Deserialize};

// use tokio::net::TcpStream;
// use tokio::io::{BufReader, AsyncRead};
// use tokio::prelude::{AsyncRead, AsyncWrite};

// use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
enum Action {
    Left,
    Right,
    Still,
    Jumping,
    Falling,
    Moving,
    Attacking,
    Blocking,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Point2<T> {
    pub x: T,
    pub y: T
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pos: Point2<f32>,
    vel: Point2<f32>,
    size: f32
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Entity {
    pub id: u8,
    facing: Action, // Left or Right
    movement: Action, // Still or Moving
    stance: Action, // Attacking, Still, or Blocking
    jumping: Action, // Jumping, Falling, or Still
    pub pos: Point2<f32>,
    pub vel: Point2<f32>,
    pub size: f32
}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            id: 0,
            facing: Action::Right, // Left or Right
            movement: Action::Still, // Still or Moving
            stance: Action::Still, // Attacking, Still, or Blocking
            jumping: Action::Still, // Jumping, Falling, or Still
            pos: Point2::<f32>{x: 0.0, y: 0.0},
            vel: Point2::<f32>{x: 0.0, y: 0.0},
            size: 0.0
        }
    }
    pub fn update_data(&mut self, id: u8, entity: Entity) {
        self.id = id;
        self.facing = entity.facing;
        self.movement = entity.movement;
        self.stance = entity.stance;
        self.jumping = entity.jumping;
        self.pos = entity.pos;
        self.vel = entity.vel;
        self.size = entity.size;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameMatch {
    pub entities: Vec<Entity>
}

impl GameMatch {
    pub fn new() -> GameMatch {
        let ent = Entity::new();
        let ent1 = Entity::new();
        let entity_vector = vec![ent, ent1];
        GameMatch {
            entities: entity_vector
        }
    }

    pub fn update_entity(&mut self, id: u8, player: Entity) {
        self.entities[id as usize].update_data(id, player);
    }
}

// impl Entity {
//     pub fn update(&mut self) {
//         match self.movement {
//             Action::Moving => self.vel.x = 1.0,
//             Action::Still => self.vel.x = 0.0,
//             _ => ()
//         };

//         match self.facing {
//             Action::Left => {
//                 if self.vel.x > 0.0 {
//                     self.vel.x = -self.vel.x;
//                 }
//             },
//             Action::Right => {
//                 if self.vel.x < 0.0 {
//                     self.vel.x = -self.vel.x
//                 }
//             }
//         }



//         self.pos.x += self.vel.x;
//         self.pos.y += self.vel.y;
//     }

//     // pub async fn from_json<R: AsyncRead>(reader: BufReader<R>) -> Entity {
//     pub async fn from_json(socket: &mut TcpStream) -> Entity{

//         let mut data = [0u8; 1024]; // byte buffer
//         let mut data_str = String::from("");

//         let byte_data = socket.read(&mut data).await;

//         let json = str::from_utf8(&byte_data).unwrap();

//         serde_json::from_str(json).unwrap()
//     }
    
//     // pub fn to_json(&self) -> String {

//     // }
// }

// pub type Entities = HashMap<u32,Entity>;