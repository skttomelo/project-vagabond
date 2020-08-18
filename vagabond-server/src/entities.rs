use serde::{Serialize, Deserialize};
use std::collections::HashMap;

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
    pub pos: Point2<f32>,
    pub vel: Point2<f32>,
    pub size: f32
}

// impl Entity {
//     pub fn to_json(&self) -> String {
//         format!("")
//     }
// }

// pub type Entities = HashMap<u32,Entity>;