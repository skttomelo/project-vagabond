use ggez;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{DrawParam, Font, Image};
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};

use serde::{Deserialize, Serialize};

use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::entity_data::Entity;
use crate::geometry::Point2;
use crate::gui_data::{create_text_with_background, Button, Clock, HealthBar};
use crate::server_data::ServerGameMatch;

// user controlled entities require this
pub trait KeyboardControlledActor {
    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool);
    fn key_up_event(&mut self, keycode: KeyCode, _keymods: KeyMods);
}
pub trait MouseControlledActor {
    fn mouse_motion_event(&mut self, x: f32, y: f32);
    fn mouse_button_up_event(&mut self, mouse_button: &MouseButton, x: f32, y: f32);
    fn mouse_button_down_event(&mut self, mouse_button: &MouseButton, x: f32, y: f32);
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RematchStatus {
    Yes,
    No,
    Maybe,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MatchStatus {
    InProgress,
    Over(usize), // player id will go in the usize
    Rematch(u8, RematchStatus),
}

pub struct GameMatch {
    pub id: usize,
    font: Font,
    clock: Clock,
    health_bar_1: HealthBar,
    health_bar_2: HealthBar,
    rematch_button: Button,
    quit_button: Button,
    match_status: MatchStatus,
    redo_status: MatchStatus,
    redoing: bool,
    pub entities: Vec<Entity>,
}

impl GameMatch {
    pub fn new(ctx: &mut Context, id: usize, font: Font) -> GameMatch {
        let ent = Entity::new(0);
        let hp_bar_1 = HealthBar::new(0);
        let ent1 = Entity::new(1);
        let hp_bar_2 = HealthBar::new(1);
        let entity_vector = vec![ent, ent1];
        let match_status = MatchStatus::InProgress;
        let redo_status = MatchStatus::InProgress;

        let mut rematch_button = Button::new(
            ctx,
            String::from("Re-match"),
            &font,
            ggez::graphics::Scale::uniform(24.0),
            Point2::new(0.0, 0.0),
            false,
        );

        rematch_button.change_location(Point2::new(
            400.0 - (rematch_button.get_text_width() / 2.0) - 10.0,
            300.0 + rematch_button.get_text_height() + 10.0,
        ));

        let mut quit_button = Button::new(
            ctx,
            String::from("Quit"),
            &font,
            ggez::graphics::Scale::uniform(24.0),
            Point2::new(0.0, 0.0),
            false,
        );

        quit_button.change_location(Point2::new(
            400.0 - (quit_button.get_text_width() / 2.0) - 10.0,
            300.0 + rematch_button.get_text_height() + 30.0 + quit_button.get_text_height(),
        ));

        GameMatch {
            id: id,
            font: font,
            clock: Clock::new(),
            health_bar_1: hp_bar_1,
            health_bar_2: hp_bar_2,
            rematch_button: rematch_button,
            quit_button: quit_button,
            match_status: match_status,
            redo_status: redo_status,
            redoing: false,
            entities: entity_vector,
        }
    }

    pub fn update(&mut self) -> GameResult {
        // update entities
        match &self.match_status {
            MatchStatus::InProgress => {
                self.entities[0].update().unwrap();
                self.entities[1].update().unwrap();
                self.rematch_button.visible = false;
                self.quit_button.visible = false;
            }
            MatchStatus::Over(_) => {
                self.rematch_button.visible = true;
                self.quit_button.visible = true;
            }
            _ => (),
        }

        // reset players positions after round restart
        if self.entities[self.id].get_hp() == 6 {
            let ent = Entity::new(self.id);

            self.entities[self.id] = ent;
        }

        self.health_bar_1.update(self.entities[0].get_hp());
        self.health_bar_2.update(self.entities[1].get_hp());

        // we can assume both buttons are visible if one is visible since they will always be visible at the same time
        if self.rematch_button.visible == true {
            if self.rematch_button.mouse_clicked() == true {
                self.redo_status = MatchStatus::Rematch(1, RematchStatus::Yes);
            } else if self.quit_button.mouse_clicked() == true {
                self.redo_status = MatchStatus::Rematch(1, RematchStatus::No);
            }
        }

        Ok(())
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        entity_spritesheet: &Image,
        entity_drawparams: &Vec<DrawParam>,
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

        // draw buttons
        self.rematch_button.draw(ctx).unwrap();
        self.quit_button.draw(ctx).unwrap();

        // draw clock
        self.clock.draw(ctx, &self.font).unwrap();

        // draw match winner text
        match self.match_status {
            MatchStatus::Over(player_id) => {
                let mut text_string = String::from("Player ");
                text_string.push_str(&player_id.to_string());
                text_string.push_str(" has won.");
                let (text, mesh) = create_text_with_background(
                    ctx,
                    text_string,
                    &self.font,
                    ggez::graphics::Scale::uniform(36.0),
                );

                let location = Point2::new(
                    (SCREEN_WIDTH / 2.0) - (text.width(ctx) as f32 / 2.0),
                    (SCREEN_HEIGHT / 2.0) - (text.height(ctx) as f32 / 2.0),
                );

                ggez::graphics::draw(ctx, &mesh, DrawParam::new().dest(location.as_mint_point()))
                    .unwrap();
                ggez::graphics::draw(ctx, &text, DrawParam::new().dest(location.as_mint_point()))
                    .unwrap();
            }
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
    pub fn get_redo_status(&self) -> MatchStatus {
        self.redo_status.clone()
    }
    pub fn get_clock(&self) -> Clock {
        self.clock.clone()
    }
    pub fn get_entities(&self) -> Vec<Entity> {
        self.entities.clone()
    }
    pub fn get_redoing(&self) -> bool {
        self.redoing.clone()
    }
}

// update GameMatch with data from ServerGameMatch
impl GameMatch {
    pub fn update_from_server_game_match(&mut self, server_game_match: &ServerGameMatch) {
        self.clock = server_game_match.get_clock();
        self.match_status = server_game_match.get_match_status();
        self.redo_status = server_game_match.get_redo_status();
        self.redoing = server_game_match.get_redoing();

        let server_entities = server_game_match.get_server_entities();

        self.entities[0].update_from_server_entity(&server_entities[0]);
        self.entities[1].update_from_server_entity(&server_entities[1]);
    }
}

impl KeyboardControlledActor for GameMatch {
    fn key_down_event(&mut self, keycode: KeyCode, keymods: KeyMods, repeat: bool) {
        &self.entities[self.id].key_down_event(keycode, keymods, repeat);
    }
    fn key_up_event(&mut self, keycode: KeyCode, keymods: KeyMods) {
        &self.entities[self.id].key_up_event(keycode, keymods);
    }
}

impl MouseControlledActor for GameMatch {
    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        let mouse_position = Point2::new(x, y);

        self.rematch_button.mouse_motion_event(&mouse_position);
        self.quit_button.mouse_motion_event(&mouse_position);
    }

    fn mouse_button_up_event(&mut self, mouse_button: &MouseButton, x: f32, y: f32) {
        let mouse_position = Point2::new(x, y);

        self.rematch_button
            .mouse_button_up_event(mouse_button, &mouse_position);
        self.quit_button
            .mouse_button_up_event(mouse_button, &mouse_position);
    }

    fn mouse_button_down_event(&mut self, mouse_button: &MouseButton, x: f32, y: f32) {
        let mouse_position = Point2::new(x, y);

        self.rematch_button
            .mouse_button_down_event(mouse_button, &mouse_position);
        self.quit_button
            .mouse_button_down_event(mouse_button, &mouse_position);
    }
}
