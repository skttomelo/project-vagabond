use serde::{Deserialize, Serialize};

use crate::geometry::{Point2, Rect};

// all possible action states for an entity to be in
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Action {
    Left,
    Right,
}

// helper struct for cleaning up Entity struct
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EntityActions {
    pub facing: Action, // Left or Right
    pub moving_left: bool,
    pub moving_right: bool,
    pub can_attack: bool,
    pub attacking: bool,
    pub damage_check: bool,
    pub blocking: bool,
}

impl EntityActions {
    pub fn new(facing: Action) -> EntityActions {
        EntityActions {
            facing: facing,
            moving_left: false,
            moving_right: false,
            can_attack: true,
            attacking: false,
            damage_check: false,
            blocking: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerEntity {
    pub id: usize,
    pub hp: i8, // health of entity
    pub entity_actions: EntityActions,
    pub pos: Point2,
    pub vel: Point2,
    pub bound: Rect,
    pub attack_bound: Rect,
}

impl ServerEntity {
    pub fn new() -> ServerEntity {
        ServerEntity {
            id: 0,
            hp: 5,
            entity_actions: EntityActions::new(Action::Right),
            pos: Point2::new(0.0,0.0),
            vel: Point2::new(0.0,0.0),
            bound: Rect::new(Point2::new(0.0,0.0), Point2::new(0.0,0.0)),
            attack_bound: Rect::new(Point2::new(0.0,0.0), Point2::new(0.0,0.0)),
        }
    }

    pub fn update_data(&mut self, id: usize, entity: ServerEntity) {
        self.id = id;
        self.hp = entity.hp;
        self.entity_actions = entity.entity_actions;
        self.pos = entity.pos;
        self.vel = entity.vel;
        self.bound = entity.bound;
        self.attack_bound = entity.attack_bound;
    }
}