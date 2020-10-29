use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Point2 {
    pub x: f32,
    pub y: f32,
}
impl Point2 {
    pub fn new(x: f32, y: f32) -> Point2 {
        Point2 { x: x, y: y }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Rect {
    pub top_left: Point2,
    pub bottom_right: Point2,
}

impl Rect {
    pub fn new(top_left: Point2, bottom_right: Point2) -> Rect {
        Rect {
            top_left: top_left,
            bottom_right: bottom_right,
        }
    }
}
