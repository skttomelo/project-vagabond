use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Point2<T> {
    pub x: T,
    pub y: T
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Entity {
    id: u8, // id will be removed... game match will handle id's instead
    facing: Action, // Left or Right
    movement: Action, // Still or Moving
    stance: Action, // Attacking, Still, or Blocking
    jumping: Action, // Jumping, Falling, or Still
    pos: Point2<f32>,
    vel: Point2<f32>,
    size: f32
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

    // basic update function (NOT FOR USE WITH VAGABOND-CLIENT)
    pub fn update(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
    }

    pub fn set_vel(&mut self, new_vel: Point2<f32>) {
        self.vel = new_vel;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameMatch {
    pub id: u8,
    pub entities: Vec<Entity>
}

impl GameMatch {
    pub fn new() -> GameMatch {
        let ent = Entity::new();
        let ent1 = Entity::new();
        let entity_vector = vec![ent, ent1];
        GameMatch {
            id: 0,
            entities: entity_vector
        }
    }
    #[allow(dead_code)]
    pub fn update_entity(&mut self, id: u8, player: Entity) {
        self.entities[id as usize].update_data(id, player);
    }
}

// data received from server
#[derive(Deserialize, Debug)]
pub struct GameMatchServer {
    pub entities: Vec<Entity>
}