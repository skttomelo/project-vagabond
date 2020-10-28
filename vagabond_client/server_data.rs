use serde::{Deserialize, Serialize};

use crate::entity_data::{Entity, EntityActions};
use crate::game_data::GameMatch;
use crate::geometry::{Point2, Rect};
use crate::gui_data::Clock;

#[derive(Deserialize, Serialize)]
pub struct ServerGameMatch {
    id: usize,
    clock: Clock,
    server_entities: Vec<ServerEntity>,
}

impl ServerGameMatch {
    pub fn from_game_match(game_match: &GameMatch) -> ServerGameMatch {
        let mut server_entities: Vec<ServerEntity> = Vec::new();
        let entities = game_match.get_entities();

        for entity in entities {
            server_entities.push(ServerEntity::from_entity(&entity));
        }

        ServerGameMatch {
            id: game_match.get_id(),
            clock: game_match.get_clock(),
            server_entities: server_entities,
        }
    }
}

#[derive(Deserialize, Serialize)]
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
    pub fn from_entity(entity: &Entity) -> ServerEntity {
        ServerEntity {
            id: entity.get_id(),
            hp: entity.get_hp(),
            entity_actions: entity.get_entity_actions(),
            pos: entity.get_pos(),
            vel: entity.get_vel(),
            bound: entity.get_bound(),
            attack_bound: entity.get_attack_bound(),
        }
    }
}
