use ggez;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{DrawParam, Font, Image};
use ggez::{Context, GameResult};

use serde::{Serialize, Deserialize};

use crate::entity_data::Entity;
use crate::gui_data::{Clock, HealthBar, create_text_with_background};
use crate::geometry::Point2;
use crate::constants::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::server_data::ServerGameMatch;

// user controlled entities require this
pub trait ControlledActor {
    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool);
    fn key_up_event(&mut self, keycode: KeyCode, _keymods: KeyMods);
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MatchStatus {
    InProgress,
    Over(usize), // player id will go in the usize
}

pub struct GameMatch {
    pub id: usize,
    clock: Clock,
    health_bar_1: HealthBar,
    health_bar_2: HealthBar,
    match_status: MatchStatus,
    pub entities: Vec<Entity>,
}

impl GameMatch {
    pub fn new(id: usize) -> GameMatch {
        let ent = Entity::new(0);
        let hp_bar_1 = HealthBar::new(0);
        let ent1 = Entity::new(1);
        let hp_bar_2 = HealthBar::new(1);
        let entity_vector = vec![ent, ent1];
        let match_status = MatchStatus::InProgress;

        GameMatch {
            id: id,
            clock: Clock::new(),
            health_bar_1: hp_bar_1,
            health_bar_2: hp_bar_2,
            match_status: match_status,
            entities: entity_vector,
        }
    }

    pub fn update(&mut self) -> GameResult {
        // update entities
        match &self.match_status {
            MatchStatus::InProgress => {
                self.entities[0].update().unwrap();
                self.entities[1].update().unwrap();
            },
            _ => ()
        }

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

        // draw match winner text
        match self.match_status {
            MatchStatus::Over(player_id) => {       
                let mut text_string = String::from("Player ");
                text_string.push_str(&player_id.to_string()); 
                text_string.push_str(" has won.");
                let (text, mesh) = create_text_with_background(ctx, text_string, font.clone(), ggez::graphics::Scale::uniform(36.0));
                
                let location = Point2::new((SCREEN_WIDTH / 2.0) - (text.width(ctx) as f32 / 2.0), (SCREEN_HEIGHT / 2.0) - (text.height(ctx) as f32 / 2.0));

                ggez::graphics::draw(ctx, &mesh, DrawParam::new().dest(location.as_mint_point())).unwrap();
                ggez::graphics::draw(ctx, &text, DrawParam::new().dest(location.as_mint_point())).unwrap();
            },
            _ => (), // match is still playing out
        }

        Ok(())
    }
}

// accessors
impl GameMatch {
    pub fn get_match_status(&self) -> MatchStatus {
        self.match_status.clone()
    }
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
        self.match_status = server_game_match.get_match_status();

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
