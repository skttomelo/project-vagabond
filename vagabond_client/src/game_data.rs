use ggez;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics;
use ggez::graphics::Color; // will be used to depict damage and enemy color
use ggez::graphics::{DrawParam, Image};
use ggez::{Context, GameResult};

use cgmath::Vector2;

use serde::{Deserialize, Serialize};

use std::time::Duration;

#[path = "animate.rs"]
mod animate;
use animate::Animator;

pub const SCALE: f32 = 5.5;
pub const TILE_SIZE: f32 = 32.0;
pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 600.0;

const PLAYER_TWO_COLOR: Color = Color::new(1.0, 0.5, 1.0, 1.0);

// user controlled entities require this
pub trait ControlledActor {
    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool);
    fn key_up_event(&mut self, keycode: KeyCode, _keymods: KeyMods);
}

// all possible action states for an entity to be in
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
enum Action {
    Left,
    Right,
    Still,
    Jumping,
    Falling,
    Attacking,
    Blocking,
    Damaged,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}
impl<T> Point2<T> {
    fn new(x: T, y: T) -> Point2<T> {
        Point2::<T> { x: x, y: y }
    }
    fn as_mint_point(&self) -> cgmath::Point2<T>
    where
        T: Copy,
    {
        cgmath::Point2::<T>::new(self.x, self.y)
    }
    fn as_mint_vector(&self) -> Vector2<T>
    where
        T: Copy,
    {
        Vector2::<T>::new(self.x, self.y)
    }
}

// Serialize, Deserialize
#[derive(Clone, Copy, Debug)]
pub struct Entity {
    id: usize,      // id will be removed... game match will handle id's instead
    facing: Action, // Left or Right
    moving_left: bool,
    moving_right: bool,
    stance: Action,  // Attacking, Still, or Blocking
    jumping: Action, // Jumping, Falling, or Still
    animator: Animator,
    pos: Point2<f32>,
    vel: Point2<f32>,
    scale: Point2<f32>, // make changes to reflect in server code because right now that is reflected as a f32 only
}
impl Entity {
    pub fn new(id: usize) -> Entity {
        let position = match id {
            0 => Point2::<f32>::new(20.0 * SCALE, SCREEN_HEIGHT - (TILE_SIZE * SCALE)),
            1 => Point2::<f32>::new(
                SCREEN_WIDTH - (20.0 * SCALE),
                SCREEN_HEIGHT - (TILE_SIZE * SCALE),
            ),
            _ => Point2::<f32>::new(0.0, 0.0),
        };

        Entity {
            id: id,
            facing: Action::Right, // Left or Right
            moving_left: false,
            moving_right: false,
            stance: Action::Still,  // Attacking, Still, or Blocking
            jumping: Action::Still, // Jumping, Falling, or Still
            pos: position,
            animator: Animator::new(3, Duration::from_millis(188)),
            vel: Point2::<f32>::new(0.0, 0.0),
            scale: Point2::<f32>::new(SCALE, SCALE),
        }
    }

    pub fn update(&mut self) -> GameResult {
        // update velocity
        self.vel.x = if self.moving_right {
            self.animator.update();
            SCALE / 2.0
        } else if self.moving_left {
            self.animator.update();
            -SCALE / 2.0
        } else {
            self.animator.end();
            0.0
        };
        // TODO: Add in jumping

        // update position
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;

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
        if self.moving_left || self.moving_right {
            // TODO add movement animation (done)
            match self.facing {
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
            // stances
            draw_param_index = match self.stance {
                Action::Blocking => 0,
                Action::Attacking => 1,
                Action::Still => 2,
                _ => 2, // because we are doing nothing
            };
        }

        let mut draw_param = entity_drawparams[draw_param_index].dest(self.pos.as_mint_point());
        if self.id == 1 {
            draw_param = draw_param
                .scale(Vector2::new(-self.scale.x, self.scale.y))
                .color(PLAYER_TWO_COLOR);
        }

        graphics::draw(ctx, entity_spritesheet, draw_param).unwrap();

        Ok(())
    }

    // might be used in the future for handling entity updates from the server
    #[allow(dead_code)]
    pub fn update_data(&mut self, id: usize, entity: Entity) {
        self.id = id;
        self.facing = entity.facing;
        self.moving_left = entity.moving_left;
        self.moving_right = entity.moving_right;
        self.stance = entity.stance;
        self.jumping = entity.jumping;
        self.pos = entity.pos;
        self.vel = entity.vel;
        self.scale = entity.scale;
    }
}

impl ControlledActor for Entity {
    fn key_down_event(&mut self, keycode: KeyCode, keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Space => self.stance = Action::Attacking,
            KeyCode::Left => {
                self.moving_left = true;
                self.facing = Action::Left;
            }
            KeyCode::Right => {
                self.moving_right = true;
                self.facing = Action::Right;
            }
            _ => (),
        };

        match keymods {
            KeyMods::SHIFT => self.stance = Action::Blocking,
            _ => (),
        };
    }
    fn key_up_event(&mut self, keycode: KeyCode, keymods: KeyMods) {
        match keycode {
            KeyCode::Space => self.stance = Action::Still,
            KeyCode::Left => self.moving_left = false,
            KeyCode::Right => self.moving_right = false,
            _ => (),
        };

        match keymods {
            KeyMods::SHIFT => self.stance = Action::Still,
            _ => (),
        };
    }
}

pub struct GameMatch {
    pub id: usize,
    pub entities: Vec<Entity>,
}

impl GameMatch {
    pub fn new() -> GameMatch {
        let ent = Entity::new(0);
        let ent1 = Entity::new(1);
        let entity_vector = vec![ent, ent1];
        GameMatch {
            id: 0,
            entities: entity_vector,
        }
    }

    pub fn update(&mut self) -> GameResult {
        // update entities
        for entity in &mut self.entities {
            entity.update().unwrap();
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

        Ok(())
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
