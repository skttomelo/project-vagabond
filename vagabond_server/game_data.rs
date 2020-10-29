use serde::{Deserialize, Serialize};

use crate::gui_data::Clock;
use crate::entity_data::ServerEntity;


#[derive(Serialize, Deserialize, Debug)]
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
        self.server_entities[id].update_data(id, player);
    }
}