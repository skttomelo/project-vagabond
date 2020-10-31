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

    pub fn as_mint_point(&self) -> cgmath::Point2<f32> {
        cgmath::Point2::<f32>::new(self.x, self.y)
    }

    #[allow(dead_code)]
    pub fn as_mint_vector(&self) -> cgmath::Vector2<f32> {
        cgmath::Vector2::<f32>::new(self.x, self.y)
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

    pub fn translate(&mut self, vel: &Point2) {
        self.top_left.x += vel.x;
        self.top_left.y += vel.y;
        self.bottom_right.x += vel.x;
        self.bottom_right.y += vel.y;
    }
}
