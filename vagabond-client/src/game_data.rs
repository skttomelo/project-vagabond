use ggez;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics;
use ggez::graphics::Image;
use ggez::graphics::Color; // will be used to depict damage and enemy color
use ggez::{Context, GameResult};

use cgmath::Vector2;

use serde::{Deserialize, Serialize};

// used with all entities (user controlled or not)
pub trait Actor {
    fn update(&mut self, _ctx: &mut Context) -> GameResult;
    fn draw(&mut self, ctx: &mut Context) -> GameResult;
}

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
    Moving,
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
    id: u8,           // id will be removed... game match will handle id's instead
    facing: Action,   // Left or Right
    movement: Action, // Still or Moving
    stance: Action,   // Attacking, Still, or Blocking
    jumping: Action,  // Jumping, Falling, or Still
    pos: Point2<f32>,
    vel: Point2<f32>,
    scale: Point2<f32>, // make changes to reflect in server code because right now that is reflected as a f32 only
}
impl Entity {
    pub fn new() -> Entity {
        Entity {
            id: 0,
            facing: Action::Right,   // Left or Right
            movement: Action::Still, // Still or Moving
            stance: Action::Still,   // Attacking, Still, or Blocking
            jumping: Action::Still,  // Jumping, Falling, or Still
            pos: Point2::<f32>::new(0.0, 0.0),
            vel: Point2::<f32>::new(0.0, 0.0),
            scale: Point2::<f32>::new(3.5, 3.5),
        }
    }

    pub fn draw(&self, ctx: &mut Context, entity_assets: &Vec<Image>) {
        // set the draw params
        let draw_param = graphics::DrawParam::new().dest(self.pos.as_mint_point()).scale(self.scale.as_mint_vector());

        graphics::draw(ctx, &entity_assets[0], draw_param).unwrap();
    }

    // might be used in the future for handling entity updates from the server
    #[allow(dead_code)]
    pub fn update_data(&mut self, id: u8, entity: Entity) {
        self.id = id;
        self.facing = entity.facing;
        self.movement = entity.movement;
        self.stance = entity.stance;
        self.jumping = entity.jumping;
        self.pos = entity.pos;
        self.vel = entity.vel;
        self.scale = entity.scale;
    }
}

// pub struct Player {
//     pos: Point2<f32>,
//     vel: Point2<f32>,
//     size: f32,
// }

// impl Player {
//     pub fn new(pos: Point2<f32>, vel: Point2<f32>, size: f32) -> Player {
//         Player { pos, vel, size }
//     }
// }

// impl Entity for Player {
//     fn update(&mut self, _ctx: &mut Context) -> GameResult {
//         self.pos.x += self.vel.x;
//         self.pos.y += self.vel.y;

//         // it worked
//         Ok(())
//     }

//     fn draw(&mut self, ctx: &mut Context) -> GameResult {
//         let circle = graphics::Mesh::new_circle(
//             ctx,
//             graphics::DrawMode::fill(),
//             self.pos,
//             self.size,
//             1.0,
//             graphics::WHITE,
//         )?;
//         graphics::draw(ctx, &circle, (self.pos,))?;

//         // it worked
//         Ok(())
//     }

//     fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
//         match keycode {
//             KeyCode::W => self.vel.y = -1.0, //negative y vel makes things move up
//             KeyCode::S => self.vel.y = 1.0,
//             KeyCode::A => self.vel.x = -1.0,
//             KeyCode::D => self.vel.x = 1.0,
//             _ => (),
//         }
//     }

//     fn key_up_event(&mut self, keycode: KeyCode, _keymods: KeyMods) {
//         match keycode {
//             KeyCode::W | KeyCode::S => self.vel.y = 0.0,
//             KeyCode::A | KeyCode::D => self.vel.x = 0.0,
//             _ => (),
//         }
//     }
// }

pub struct GameMatch {
    pub id: u8,
    pub entities: Vec<Entity>,
}

impl GameMatch {
    pub fn new() -> GameMatch {
        let ent = Entity::new();
        let ent1 = Entity::new();
        let entity_vector = vec![ent, ent1];
        GameMatch {
            id: 0,
            entities: entity_vector,
        }
    }
    pub fn draw(&mut self, ctx: &mut Context, entity_assets: &Vec<Image>) -> GameResult {
        // draw entities
        for entity in &self.entities {
            entity.draw(ctx, entity_assets);
        }

        Ok(())
    }
}

// data received from server
#[derive(Deserialize, Debug)]
pub struct GameMatchServer {
    pub entities: Vec<Entity>,
}
