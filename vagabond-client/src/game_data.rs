use ggez;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics;
use ggez::graphics::Color; // will be used to depict damage and enemy color
use ggez::graphics::Image;
use ggez::{Context, GameResult};

use cgmath::Vector2;

use serde::{Deserialize, Serialize};

pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 600.0;
const TILE_SIZE: f32 = 32.0;
const SCALE: f32 = 5.5;
const PLAYER_TWO_COLOR: Color = Color::new(1.0, 0.5, 0.0, 1.0);

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

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Entity {
    id: usize,      // id will be removed... game match will handle id's instead
    facing: Action, // Left or Right
    moving_left: bool,
    moving_right: bool,
    stance: Action,  // Attacking, Still, or Blocking
    jumping: Action, // Jumping, Falling, or Still
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
            vel: Point2::<f32>::new(0.0, 0.0),
            scale: Point2::<f32>::new(SCALE, SCALE),
        }
    }

    pub fn update(&mut self) -> GameResult {
        // update velocity
        self.vel.x = if self.moving_right {
            SCALE / 2.0
        } else if self.moving_left {
            -SCALE / 2.0
        } else {
            0.0
        };
        // TODO: Add in jumping

        // update position
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;

        Ok(())
    }

    pub fn draw(&self, ctx: &mut Context, entity_assets: &Vec<Image>) -> GameResult {
        // set the draw params
        let mut draw_param = graphics::DrawParam::new()
            .dest(self.pos.as_mint_point())
            .scale(self.scale.as_mint_vector());
        if self.id == 1 {
            draw_param = draw_param
                .scale(Vector2::new(-self.scale.x, self.scale.y))
                .color(PLAYER_TWO_COLOR);
        }

        // draw stance / animation depending on Action
        if self.moving_left || self.moving_right {
            // TODO add movement animation
            match self.facing {
                _ => (),
            };
        }

        // stances
        match self.stance {
            Action::Blocking => graphics::draw(ctx, &entity_assets[0], draw_param).unwrap(),
            Action::Attacking => graphics::draw(ctx, &entity_assets[5], draw_param).unwrap(),
            Action::Still => graphics::draw(ctx, &entity_assets[2], draw_param).unwrap(),
            _ => (),
        };

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

    pub fn draw(&mut self, ctx: &mut Context, entity_assets: &Vec<Image>) -> GameResult {
        // draw entities
        for entity in &self.entities {
            entity.draw(ctx, entity_assets).unwrap();
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
#[derive(Deserialize, Debug)]
pub struct GameMatchServer {
    pub entities: Vec<Entity>,
}
