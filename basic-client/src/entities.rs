use serde::{Serialize, Deserialize};

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

impl Player {
    pub fn new(pos: Point2<f32>, vel: Point2<f32>, size: f32) -> Player {
        Player{
            pos,
            vel,
            size
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Entity {
    id: u8,
    facing: Action, // Left or Right
    movement: Action, // Still or Moving
    stance: Action, // Attacking, Still, or Blocking
    jumping: Action, // Jumping, Falling, or Still
    pos: Point2<f32>,
    vel: Point2<f32>,
    size: f32
}