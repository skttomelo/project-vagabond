use ggez;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{DrawParam, Font, Image};
use ggez::{Context, GameResult};

use crate::entity_data::Entity;
use crate::gui_data::{Clock, HealthBar};
use crate::server_data::ServerGameMatch;

// user controlled entities require this
pub trait ControlledActor {
    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool);
    fn key_up_event(&mut self, keycode: KeyCode, _keymods: KeyMods);
}

pub struct GameMatch {
    pub id: usize,
    clock: Clock,
    health_bar_1: HealthBar,
    health_bar_2: HealthBar,
    pub entities: Vec<Entity>,
}

impl GameMatch {
    pub fn new(id: usize) -> GameMatch {
        let ent = Entity::new(0);
        let hp_bar_1 = HealthBar::new(0);
        let ent1 = Entity::new(1);
        let hp_bar_2 = HealthBar::new(1);
        let entity_vector = vec![ent, ent1];
        GameMatch {
            id: id,
            clock: Clock::new(),
            health_bar_1: hp_bar_1,
            health_bar_2: hp_bar_2,
            entities: entity_vector,
        }
    }

    pub fn update(&mut self) -> GameResult {
        // update entities
        self.entities[0].update().unwrap();
        self.entities[1].update().unwrap();

        // player 1 is attacking
        self.attack_bound_check(0, 1);

        // player 2 is attacking
        self.attack_bound_check(1, 0);

        self.health_bar_1.update(self.entities[0].get_hp());
        self.health_bar_2.update(self.entities[1].get_hp());

        Ok(())
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        entity_spritesheet: &Image,
        entity_drawparams: &Vec<DrawParam>,
        font: &Font,
    ) -> GameResult {
        // draw entities
        for entity in &self.entities {
            entity
                .draw(ctx, entity_spritesheet, entity_drawparams)
                .unwrap();
        }

        // draw health bars
        self.health_bar_1.draw(ctx).unwrap();
        self.health_bar_2.draw(ctx).unwrap();

        // draw clock

        self.clock.draw(ctx, font).unwrap();

        Ok(())
    }

    fn attack_bound_check(&mut self, first_entity_id: usize, second_entity_id: usize) {
        if self.entities[first_entity_id]
            .get_entity_actions_as_ref()
            .damage_check
            == true
            && self.entities[first_entity_id]
                .get_entity_actions_as_ref()
                .blocking
                == false
            && self.entities[second_entity_id]
                .get_entity_actions_as_ref()
                .blocking
                == false
        {
            if self.entities[first_entity_id]
                .get_attack_bound()
                .check_bounds(&self.entities[second_entity_id].get_bound())
                == true
            {
                self.entities[second_entity_id].take_damage(1);
            }
        }
        self.entities[first_entity_id]
            .get_entity_actions_as_mut_ref()
            .damage_check = false;
    }
}

// accessors
impl GameMatch {
    pub fn get_clock(&self) -> Clock {
        self.clock.clone()
    }
    pub fn get_entities(&self) -> Vec<Entity> {
        self.entities.clone()
    }
}

// update GameMatch with data from ServerGameMatch
impl GameMatch {
    pub fn update_from_server_game_match(&mut self, server_game_match: &ServerGameMatch) {
        self.clock = server_game_match.get_clock();

        let server_entities = server_game_match.get_server_entities();

        self.entities[0].update_from_server_entity(&server_entities[0]);
        self.entities[1].update_from_server_entity(&server_entities[1]);
    }
}

impl ControlledActor for GameMatch {
    fn key_down_event(&mut self, keycode: KeyCode, keymods: KeyMods, repeat: bool) {
        &self.entities[self.id].key_down_event(keycode, keymods, repeat);
    }
    fn key_up_event(&mut self, keycode: KeyCode, keymods: KeyMods) {
        &self.entities[self.id].key_up_event(keycode, keymods);
    }
}
