use ggez;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{DrawParam, Font, Image};
use ggez::{Context, GameResult};

use serde::{Deserialize, Serialize};

use crate::entity_data::Entity;
use crate::gui_data::{Clock, HealthBar};

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
    pub fn new() -> GameMatch {
        let ent = Entity::new(0);
        let hp_bar_1 = HealthBar::new(0);
        let ent1 = Entity::new(1);
        let hp_bar_2 = HealthBar::new(1);
        let entity_vector = vec![ent, ent1];
        GameMatch {
            id: 0,
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
        if self.entities[first_entity_id].entity_actions.damage_check == true
        && self.entities[first_entity_id].entity_actions.blocking == false
        && self.entities[second_entity_id].entity_actions.blocking == false
        {
            if self.entities[first_entity_id]
                .attack_bound
                .check_bounds(&self.entities[second_entity_id].bound)
                == true
            {
                self.entities[second_entity_id].take_damage();
            }
        }
        self.entities[first_entity_id].entity_actions.damage_check = false;
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

// data received from server
// Deserialize
#[derive(Debug)]
pub struct GameMatchServer {
    pub entities: Vec<Entity>,
}
