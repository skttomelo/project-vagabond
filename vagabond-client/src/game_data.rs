use ggez;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics;
use ggez::graphics::Image;
use ggez::graphics::Color; // will be used to depict damage and enemy color
use ggez::{Context, GameResult};

use cgmath::Vector2;

use serde::{Deserialize, Serialize};

const SCREEN_WIDTH:f32 = 800.0;
const SCREEN_HEIGHT:f32 = 600.0;
const TILE_SIZE:f32 = 32.0;
const SCALE:f32 = 5.5;
const PLAYER_TWO_COLOR:Color = Color::new(1.0,0.5,0.0,1.0);


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
    id: usize,           // id will be removed... game match will handle id's instead
    facing: Action,   // Left or Right
    movement: Action, // Still or Moving
    stance: Action,   // Attacking, Still, or Blocking
    jumping: Action,  // Jumping, Falling, or Still
    pos: Point2<f32>,
    vel: Point2<f32>,
    scale: Point2<f32>, // make changes to reflect in server code because right now that is reflected as a f32 only
}
impl Entity {
    pub fn new(id: usize) -> Entity {
        let position = match id {
            0 => Point2::<f32>::new(20.0*SCALE, SCREEN_HEIGHT-(TILE_SIZE*SCALE)),
            1 => Point2::<f32>::new(SCREEN_WIDTH-(20.0*SCALE), SCREEN_HEIGHT-(TILE_SIZE*SCALE)),
            _ => Point2::<f32>::new(0.0, 0.0)
        };

        Entity {
            id: id,
            facing: Action::Right,   // Left or Right
            movement: Action::Still, // Still or Moving
            stance: Action::Still,   // Attacking, Still, or Blocking
            jumping: Action::Still,  // Jumping, Falling, or Still
            pos: position,
            vel: Point2::<f32>::new(0.0, 0.0),
            scale: Point2::<f32>::new(SCALE, SCALE),
        }
    }

    pub fn update(&mut self) -> GameResult {

        // update velocity
        self.vel.x = match self.movement {
            Action::Moving => {
                match self.facing {
                    Action::Left => -1.0,
                    Action::Right => 1.0,
                    _ => 0.0 // This will never happen
                }
            },
            Action::Still => 0.0,
            _ => 0.0 // This will never happen
        };
        // TODO: Add in jumping

        // update position
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;

        Ok(())
    }

    pub fn draw(&self, ctx: &mut Context, entity_assets: &Vec<Image>) {
        // set the draw params
        let mut draw_param = graphics::DrawParam::new().dest(self.pos.as_mint_point()).scale(self.scale.as_mint_vector());
        if self.id == 1 {
            draw_param = draw_param.scale(Vector2::new(-self.scale.x, self.scale.y)).color(PLAYER_TWO_COLOR);
        }

        graphics::draw(ctx, &entity_assets[0], draw_param).unwrap();
    }

    // might be used in the future for handling entity updates from the server
    #[allow(dead_code)]
    pub fn update_data(&mut self, id: usize, entity: Entity) {
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

impl ControlledActor for Entity {
    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            // KeyCode::W => self.vel.y = -1.0, //negative y vel makes things move up
            // KeyCode::S => self.vel.y = 1.0,
            KeyCode::A => { // moving left
                self.movement = Action::Moving;
                self.facing = Action::Left;
            },
            KeyCode::D => { // moving right
                self.movement = Action::Moving;
                self.facing = Action::Right;
            },
            _ => (),
        }
    }
    fn key_up_event(&mut self, keycode: KeyCode, _keymods: KeyMods) {
        match keycode {
            KeyCode::A => {
                self.movement = Action::Still;
            },
            KeyCode::D => {
                self.movement = Action::Still;
            },
            _ => (),
        }
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
            entity.draw(ctx, entity_assets);
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
