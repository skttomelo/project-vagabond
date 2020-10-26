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

// all possible action states for an entity to be in
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Action {
    Left,
    Right,
    Still,
    Jumping,
    Falling,
    Damaged,
}

// helper struct for cleaning up Entity struct
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct EntityActions {
    pub facing: Action, // Left or Right
    pub moving_left: bool,
    pub moving_right: bool,
    pub attacking: bool,
    pub blocking: bool,
    pub jumping: Action, // Jumping, Falling, or Still
}

impl EntityActions {
    pub fn new(facing: Action) -> EntityActions {
        EntityActions {
            facing: facing,
            moving_left: false,
            moving_right: false,
            attacking: false,
            blocking: false,
            jumping: Action::Still,
        }
    }
}

// Serialize, Deserialize -- not needed because there will be a struct that will be used for sending to the server that is not this
#[derive(Clone, Copy, Debug)]
pub struct Entity {
    id: usize, // id will be removed... game match will handle id's instead
    hp: u8,    // health of entity
    dmg: u8,   // damage the entity can deal
    pub entity_actions: EntityActions,
    animator: Animator,
    pos: Point2,
    vel: Point2,
    pub bound: Rect,
    pub attack_bound: Rect,
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
            dmg: 1,
            entity_actions: EntityActions::new(facing),
            pos: position,
            animator: Animator::new(3, Duration::from_millis(188), true),
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
                self.animator.update();
                SCALE / 2.0
            } else if self.entity_actions.moving_left {
                self.animator.update();
                -SCALE / 2.0
            } else {
                self.animator.end();
                0.0
            };
        } else {
            self.animator.end();
            self.vel.x = 0.0;
        }

        // update position
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;

        self.bound.translate(&self.vel);
        self.attack_bound.translate(&self.vel);

        Ok(())
    }

    /***************************************************************************************
     * TODO: factor out calculations and assignments to update function from draw function *
     ***************************************************************************************/
    pub fn draw(
        &self,
        ctx: &mut Context,
        entity_spritesheet: &Image,
        entity_drawparams: &Vec<DrawParam>,
    ) -> GameResult {
        let mut draw_param_index = 0usize;

        // movement animation
        if self.entity_actions.attacking || self.entity_actions.blocking {
            if self.entity_actions.blocking {
                draw_param_index = 0;
            } else if self.entity_actions.attacking {
                draw_param_index = 1;
            }
        } else if self.entity_actions.moving_left || self.entity_actions.moving_right {
            match self.entity_actions.facing {
                Action::Right => {
                    // moving to the right
                    draw_param_index = 3 + self.animator.current_frame();
                }
                Action::Left => {
                    // moving to the left
                    draw_param_index = 5 - self.animator.current_frame();
                }
                _ => (),
            };
        } else {
            // not moving or attacking or blocking
            draw_param_index = 2;
        }

        let mut draw_param = entity_drawparams[draw_param_index].dest(self.pos.as_mint_point());
        if self.id == 1 {
            // we are facing the left and thus need to invert the scale
            draw_param = draw_param
                .scale(Vector2::new(-self.scale.x, self.scale.y))
                .color(PLAYER_TWO_COLOR);
        }

        // let random_rect = graphics::Rect::new(self.bound.top_left.x, self.bound.top_left.y, TILE_SIZE*SCALE, TILE_SIZE*SCALE);
        // let random_attack_rect = graphics::Rect::new(self.attack_bound.top_left.x, self.attack_bound.top_left.y, self.attack_bound.bottom_right.x - self.attack_bound.top_left.x, self.attack_bound.bottom_right.y - self.attack_bound.top_left.y);
        // let random_mesh = graphics::MeshBuilder::new().rectangle(graphics::DrawMode::Fill(graphics::FillOptions::default()), random_rect, Color::new(0.0,1.0,0.0,1.0)).build(ctx).unwrap();
        // let random_attack_mesh = graphics::MeshBuilder::new().rectangle(graphics::DrawMode::Fill(graphics::FillOptions::default()), random_attack_rect, Color::new(1.0,0.0,0.0,1.0)).build(ctx).unwrap();
        // graphics::draw(ctx, &random_mesh, DrawParam::new()).unwrap();
        // graphics::draw(ctx, &random_attack_mesh, DrawParam::new()).unwrap();

        graphics::draw(ctx, entity_spritesheet, draw_param).unwrap();

        Ok(())
    }

    pub fn get_hp(&self) -> u8 {
        self.hp
    }

    pub fn take_damage(&mut self) {
        self.hp -= self.dmg;
    }

    // might be used in the future for handling entity updates from the server
    #[allow(dead_code)]
    pub fn update_data(&mut self, id: usize, entity: Entity) {
        self.id = id;
        self.entity_actions.facing = entity.entity_actions.facing;
        self.entity_actions.moving_left = entity.entity_actions.moving_left;
        self.entity_actions.moving_right = entity.entity_actions.moving_right;
        self.entity_actions.attacking = entity.entity_actions.attacking;
        self.entity_actions.blocking = entity.entity_actions.blocking;
        self.entity_actions.jumping = entity.entity_actions.jumping;
        self.pos = entity.pos;
        self.vel = entity.vel;
        self.scale = entity.scale;
    }
}

impl ControlledActor for Entity {
    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Space => self.entity_actions.attacking = true,
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
            KeyCode::Space => self.entity_actions.attacking = false,
            KeyCode::Left => self.entity_actions.moving_left = false,
            KeyCode::Right => self.entity_actions.moving_right = false,
            KeyCode::Down => self.entity_actions.blocking = false,
            _ => (),
        };
    }
}
