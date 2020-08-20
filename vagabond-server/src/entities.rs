use std::str;

use serde::{Serialize, Deserialize};

use tokio::io::{ReadHalf, AsyncRead, AsyncWrite};

// use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    pub id: u32,
    facing: Action, // Left or Right
    movement: Action, // Still or Moving
    stance: Action, // Attacking, Still, or Blocking
    jumping: Action, // Jumping, Falling, or Still
    pos: Point2<f32>,
    vel: Point2<f32>,
    pub size: f32
}

impl Entity {
    pub fn update(&mut self) {
        match self.movement {
            Action::Moving => self.vel.x = 1.0,
            Action::Still => self.vel.x = 0.0,
            _ => ()
        };

        match self.facing {
            Action::Left => {
                if self.vel.x > 0.0 {
                    self.vel.x = -self.vel.x;
                }
            },
            Action::Right => {
                if self.vel.x < 0.0 {
                    self.vel.x = -self.vel.x
                }
            }
        }



        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
    }

    pub fn from_json(reader: &TcpStream) -> Entity {
        let mut data = [0 as u8; 1024]; // byte buffer
        let byte_data = reader.poll_read(&mut data);
        let json = str::from_utf8(&byte_data).unwrap();

        serde_json::from_str(json).unwrap()
    }
    
    // pub fn to_json(&self) -> String {

    // }
}

// pub type Entities = HashMap<u32,Entity>;