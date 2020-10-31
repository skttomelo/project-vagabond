use serde::{Deserialize, Serialize};

use crate::geometry::{Point2, Rect};

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerGameMatch {
    pub clock: Clock,
    pub server_entities: Vec<ServerEntity>,
}

impl ServerGameMatch {
    pub fn new() -> ServerGameMatch {
        let ent = ServerEntity::new();
        let ent1 = ServerEntity::new();
        let entity_vector = vec![ent, ent1];
        ServerGameMatch {
            clock: Clock::new(),
            server_entities: entity_vector,
        }
    }

    pub fn update_entity(&mut self, id: usize, player: ServerEntity) {
        self.server_entities[id] = player;
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ServerEntity {
    id: usize,
    hp: i8, // health of entity
    entity_actions: EntityActions,
    pos: Point2,
    vel: Point2,
    bound: Rect,
    attack_bound: Rect,
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
}

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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Clock {
    current: u16,
}
impl Clock {
    pub fn new() -> Clock {
        Clock { current: 0 }
    }
}
