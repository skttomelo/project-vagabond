use ggez;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics;
use ggez::graphics::{DrawParam, Image};
use ggez::{Context, GameResult};

use cgmath::Vector2;

use serde::{Deserialize, Serialize};

use std::time::Duration;

use crate::animate::Animator;
use crate::constants::{MAX_HP, PLAYER_TWO_COLOR, SCALE, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};
use crate::game_data::ControlledActor;
use crate::geometry::{Point2, Rect};
use crate::server_data::ServerEntity;

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

// Serialize, Deserialize -- not needed because there will be a struct that will be used for sending to the server that is not this
#[derive(Clone, Debug)]
pub struct Entity {
    id: usize,
    hp: i8, // health of entity
    entity_actions: EntityActions,
    movement_animator: Animator,
    attack_animator: Animator,
    pos: Point2,
    vel: Point2,
    bound: Rect,
    attack_bound: Rect,
    scale: Point2, // make changes to reflect in server code because right now that is reflected as a f32 only
}
impl Entity {
    pub fn new(id: usize) -> Entity {
        // entity location
        let position = match id {
            1 => Point2::new(
                SCREEN_WIDTH - (20.0 * SCALE),
                SCREEN_HEIGHT - (TILE_SIZE * SCALE),
            ),
            _ => Point2::new(20.0 * SCALE, SCREEN_HEIGHT - (TILE_SIZE * SCALE)),
        };

        // Rect.top_left location
        let bound_top_left_position = match id {
            1 => {
                let mut pos = position.clone();
                pos.x -= TILE_SIZE * SCALE;
                pos
            }
            _ => position.clone(),
        };

        // Rect.bottom_right location
        let bound_bottom_right_position = Point2::new(
            bound_top_left_position.x + (TILE_SIZE * SCALE),
            bound_top_left_position.y + (TILE_SIZE * SCALE),
        );

        // Attack Rect.top_left location
        let attack_top_left_position = match id {
            1 => Point2::new(
                bound_top_left_position.x,
                bound_top_left_position.y + (6.0 * SCALE),
            ),
            _ => Point2::new(
                bound_top_left_position.x + (20.0 * SCALE),
                bound_top_left_position.y + (6.0 * SCALE),
            ),
        };

        // Attack Rect.bottom_right location
        let attack_bottom_right_position = match id {
            1 => Point2::new(
                bound_top_left_position.x + (12.0 * SCALE),
                bound_top_left_position.y + (17.0 * SCALE),
            ),
            _ => Point2::new(
                bound_bottom_right_position.x,
                bound_top_left_position.y + (17.0 * SCALE),
            ),
        };

        let facing = match id {
            0 => Action::Right,
            1 => Action::Left,
            _ => Action::Right,
        };

        Entity {
            id: id,
            hp: MAX_HP,
            entity_actions: EntityActions::new(facing),
            pos: position,
            movement_animator: Animator::new(3, Duration::from_millis(148), -1),
            attack_animator: Animator::new(3, Duration::from_millis(100), 1),
            vel: Point2::new(0.0, 0.0),
            bound: Rect::new(bound_top_left_position, bound_bottom_right_position),
            attack_bound: Rect::new(attack_top_left_position, attack_bottom_right_position),
            scale: Point2::new(SCALE, SCALE),
        }
    }

    pub fn update(&mut self) -> GameResult {
        // update velocity
        if self.entity_actions.attacking == false && self.entity_actions.blocking == false {
            self.vel.x = if self.entity_actions.moving_right {
                self.movement_animator.update();
                SCALE / 2.0
            } else if self.entity_actions.moving_left {
                self.movement_animator.update();
                -SCALE / 2.0
            } else {
                self.movement_animator.end();
                0.0
            };
        } else {
            self.movement_animator.end();

            if self.entity_actions.attacking == true {
                if self.attack_animator.current_repeat() > self.attack_animator.max_repeats() {
                    self.entity_actions.damage_check = true;
                    self.entity_actions.attacking = false;
                    self.entity_actions.can_attack = true;
                } else {
                    self.attack_animator.update();
                }
            }

            self.vel.x = 0.0;
        }

        // update position
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;

        self.bound.translate(&self.vel);
        self.attack_bound.translate(&self.vel);

        Ok(())
    }

    pub fn draw(
        &self,
        ctx: &mut Context,
        entity_spritesheet: &Image,
        entity_drawparams: &Vec<DrawParam>,
    ) -> GameResult {
        let draw_param_index = self.get_draw_param();

        let mut draw_param = entity_drawparams[draw_param_index].dest(self.pos.as_mint_point());
        if self.id == 1 {
            // we are facing the left and thus need to invert the scale
            draw_param = draw_param
                .scale(Vector2::new(-self.scale.x, self.scale.y))
                .color(PLAYER_TWO_COLOR);
        }

        // let random_rect = graphics::Rect::new(self.bound.top_left.x, self.bound.top_left.y, TILE_SIZE*SCALE, TILE_SIZE*SCALE);
        // let random_attack_rect = graphics::Rect::new(self.attack_bound.top_left.x, self.attack_bound.top_left.y, self.attack_bound.bottom_right.x - self.attack_bound.top_left.x, self.attack_bound.bottom_right.y - self.attack_bound.top_left.y);
        // let random_mesh = graphics::MeshBuilder::new().rectangle(graphics::DrawMode::Fill(graphics::FillOptions::default()), random_rect, graphics::Color::new(0.0,1.0,0.0,1.0)).build(ctx).unwrap();
        // let random_attack_mesh = graphics::MeshBuilder::new().rectangle(graphics::DrawMode::Fill(graphics::FillOptions::default()), random_attack_rect, graphics::Color::new(1.0,0.0,0.0,1.0)).build(ctx).unwrap();
        // graphics::draw(ctx, &random_mesh, DrawParam::new()).unwrap();
        // graphics::draw(ctx, &random_attack_mesh, DrawParam::new()).unwrap();

        graphics::draw(ctx, entity_spritesheet, draw_param).unwrap();

        Ok(())
    }

    fn get_draw_param(&self) -> usize {
        // movement animation
        if self.entity_actions.attacking {
            if self.attack_animator.current_repeat() < self.attack_animator.max_repeats() {
                return 1 + self.attack_animator.current_frame(); // raising sword
            } else if self.attack_animator.current_repeat() == self.attack_animator.max_repeats() {
                return 3 - self.attack_animator.current_frame(); // lowering sword
            } else {
                return 1;
            }
        } else if self.entity_actions.blocking {
            return 0;
        } else if self.entity_actions.moving_left || self.entity_actions.moving_right {
            match self.entity_actions.facing {
                Action::Right => {
                    // moving to the right
                    return 5 + self.movement_animator.current_frame();
                }
                _ => {
                    // moving to the left
                    return 7 - self.movement_animator.current_frame();
                }
            };
        } else {
            // not moving or attacking or blocking
            return 4;
        }
    }
}

// accessors and one mutator
impl Entity {
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

    // might implement
    pub fn get_movement_animator(&self) -> Animator {
        self.movement_animator.clone()
    }

    // might implement
    pub fn get_attack_animator(&self) -> Animator {
        self.attack_animator.clone()
    }

    pub fn get_entity_actions(&self) -> EntityActions {
        self.entity_actions.clone()
    }
}

// update data received from the server
impl Entity {
    pub fn update_from_server_entity(&mut self, server_entity: &ServerEntity) {
        self.id = server_entity.get_id();
        self.hp = server_entity.get_hp();
        self.entity_actions = server_entity.get_entity_actions();
        self.attack_animator.update_from_server_animator(&server_entity.get_attack_animator());
        self.pos = server_entity.get_pos();
        self.vel = server_entity.get_vel();
        self.bound = server_entity.get_bound();
        self.attack_bound = server_entity.get_attack_bound();
    }
}

impl ControlledActor for Entity {
    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Space => {
                if self.entity_actions.can_attack {
                    self.entity_actions.attacking = true;
                    self.attack_animator.end();
                }
                self.entity_actions.can_attack = false;
            }
            KeyCode::Left => {
                self.entity_actions.moving_left = true;
                self.entity_actions.facing = Action::Left;
            }
            KeyCode::Right => {
                self.entity_actions.moving_right = true;
                self.entity_actions.facing = Action::Right;
            }
            KeyCode::Down => self.entity_actions.blocking = true,
            _ => (),
        };
    }
    fn key_up_event(&mut self, keycode: KeyCode, _keymods: KeyMods) {
        match keycode {
            KeyCode::Left => self.entity_actions.moving_left = false,
            KeyCode::Right => self.entity_actions.moving_right = false,
            KeyCode::Down => self.entity_actions.blocking = false,
            _ => (),
        };
    }
}
