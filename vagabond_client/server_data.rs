use serde::{Deserialize, Serialize};

use crate::animate::Animator;
use crate::entity_data::{Entity, EntityActions};
use crate::game_data::{GameMatch, MatchStatus};
use crate::geometry::{Point2, Rect};
use crate::gui_data::Clock;

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerGameMatch {
    clock: Clock,
    server_entities: Vec<ServerEntity>,
    match_status: MatchStatus,
    redo_status: MatchStatus,
    redoing: bool,
}

impl ServerGameMatch {
    pub fn from_game_match(game_match: &GameMatch) -> ServerGameMatch {
        let mut server_entities: Vec<ServerEntity> = Vec::new();
        let entities = game_match.get_entities();
        let match_status = game_match.get_match_status();
        let redo_status = game_match.get_redo_status();
        let redoing = game_match.get_redoing();

        for entity in entities {
            server_entities.push(ServerEntity::from_entity(&entity));
        }

        ServerGameMatch {
            clock: game_match.get_clock(),
            server_entities: server_entities,
            match_status: match_status,
            redo_status: redo_status,
            redoing: redoing,
        }
    }
}

// accessors
impl ServerGameMatch {
    pub fn get_clock(&self) -> Clock {
        self.clock.clone()
    }
    pub fn get_match_status(&self) -> MatchStatus {
        self.match_status.clone()
    }
    pub fn get_redo_status(&self) -> MatchStatus {
        self.redo_status.clone()
    }
    pub fn get_server_entities(&self) -> Vec<ServerEntity> {
        self.server_entities.clone()
    }
    pub fn get_redoing(&self) -> bool {
        self.redoing.clone()
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ServerEntity {
    id: usize,
    hp: i8, // health of entity
    entity_actions: EntityActions,
    attack_animator: ServerAnimator,
    pos: Point2,
    vel: Point2,
    bound: Rect,
    attack_bound: Rect,
}

impl ServerEntity {
    pub fn from_entity(entity: &Entity) -> ServerEntity {
        ServerEntity {
            id: entity.get_id(),
            hp: entity.get_hp(),
            entity_actions: entity.get_entity_actions(),
            attack_animator: ServerAnimator::from_animator(&entity.get_attack_animator()),
            pos: entity.get_pos(),
            vel: entity.get_vel(),
            bound: entity.get_bound(),
            attack_bound: entity.get_attack_bound(),
        }
    }
}

// accessors
impl ServerEntity {
    pub fn get_id(&self) -> usize {
        self.id
    }
    pub fn get_hp(&self) -> i8 {
        self.hp
    }

    pub fn get_pos(&self) -> Point2 {
        self.pos.clone()
    }

    pub fn get_vel(&self) -> Point2 {
        self.vel.clone()
    }

    pub fn get_bound(&self) -> Rect {
        self.bound.clone()
    }

    pub fn get_attack_bound(&self) -> Rect {
        self.attack_bound.clone()
    }

    pub fn get_attack_animator(&self) -> ServerAnimator {
        self.attack_animator.clone()
    }

    pub fn get_entity_actions(&self) -> EntityActions {
        self.entity_actions.clone()
    }
}

// server will not handle animation timing
// rather it will facillitate the syncing
// of frames
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerAnimator {
    current_frame: u64,
    current_repeat: i8,
}

impl ServerAnimator {
    pub fn from_animator(animator: &Animator) -> ServerAnimator {
        ServerAnimator {
            current_frame: animator.current_frame() as u64,
            current_repeat: animator.current_repeat(),
        }
    }
}

impl ServerAnimator {
    pub fn current_frame(&self) -> usize {
        self.current_frame as usize
    }

    pub fn current_repeat(&self) -> i8 {
        self.current_repeat
    }
}
