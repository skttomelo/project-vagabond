
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Clock {
    current: u16,
}
impl Clock {
    pub fn new() -> Clock {
        Clock { current: 0 }
    }
}
