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

    // checks to see if two Rects are intersecting
    // using solution found here: https://www.geeksforgeeks.org/find-two-rectangles-overlap/
    // however I had to make an adjustment because their bound check for the Y-axis was wrong
    pub fn check_bounds(self, rect: &Rect) -> bool {
        // if one rectangle is on left side of other
        if self.top_left.x >= rect.bottom_right.x || rect.top_left.x >= self.bottom_right.x {
            return false;
        }

        // if one rectangle is above other
        if self.top_left.y >= rect.bottom_right.y || rect.top_left.y >= self.bottom_right.y {
            return false;
        }

        true
    }
}
