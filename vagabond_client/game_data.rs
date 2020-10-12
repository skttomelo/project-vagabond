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

#[path = "geometry.rs"]
mod geometry;
use geometry::{Point2, Rect};

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

struct HealthBar {
    id: usize, // player id
    max_hp: i8,
    foreground_rectangle: graphics::Rect,
    background_rectangle: graphics::Rect,
    foreground_color: Color,
    background_color: Color,
    draw_param: DrawParam,
}
impl HealthBar {
    pub fn new(id: usize) -> HealthBar {
        let width = match id {
            1 => -10.0 * SCALE * 4.0,
            _ => 10.0 * SCALE * 4.0
        };
        let height = 3.0 * SCALE * 2.0;
        let f_color = Color::new(1.0, 0.0, 0.0, 1.0);
        let b_color = Color::new(0.5, 0.5, 0.5, 0.5);

        // id corresponds to the player
        let position = match id {
            1 => Point2::new(SCREEN_WIDTH, 0.0),
            _ => Point2::new(0.0, 0.0)
        };

        let f_rect = graphics::Rect::new(position.x, position.y, width, height);
        let b_rect = graphics::Rect::new(position.x, position.y, width, height);

        let draw_param = DrawParam::new().src(f_rect.clone());

        HealthBar {
            id: id,
            max_hp: 5,
            foreground_rectangle: f_rect,
            background_rectangle: b_rect,
            foreground_color: f_color,
            background_color: b_color,
            draw_param: draw_param,
        }
    }

    pub fn update(&mut self, current_hp: i8) {
        let new_width = (self.background_rectangle.w / self.max_hp as f32) * current_hp as f32;
        self.foreground_rectangle.w = new_width;
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {

        self.foreground_rectangle = self.background_rectangle.clone();
        
        self.draw_param = DrawParam::new().src(self.foreground_rectangle.clone());
        if self.id == 1 {
            self.draw_param = self.draw_param.scale(Vector2::<f32>::new(-1.0,1.0))
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let mut mesh_builder = graphics::MeshBuilder::new();
        
        // draw background first
        let b_mesh = mesh_builder.rectangle(graphics::DrawMode::Fill(graphics::FillOptions::default()), self.background_rectangle, self.background_color).build(ctx).unwrap();

        // draw foreground
        let f_mesh = mesh_builder.rectangle(graphics::DrawMode::Fill(graphics::FillOptions::default()), self.foreground_rectangle, self.foreground_color).build(ctx).unwrap();


        graphics::draw(ctx, &b_mesh, self.draw_param.color(self.background_color)).unwrap();
        graphics::draw(ctx, &f_mesh, self.draw_param.color(self.foreground_color)).unwrap();

        Ok(())
    }
}

// all possible action states for an entity to be in
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
enum Action {
    Left,
    Right,
    Still,
    Jumping,
    Falling,
    Damaged,
}

// helper struct for cleaning up Entity struct
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
struct EntityActions {
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
    hp: i8,    // health of entity
    dmg: i8,   // damage the entity can deal
    entity_actions: EntityActions,
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
            0 => Point2::new(20.0 * SCALE, SCREEN_HEIGHT - (TILE_SIZE * SCALE)),
            1 => Point2::new(
                SCREEN_WIDTH - (20.0 * SCALE),
                SCREEN_HEIGHT - (TILE_SIZE * SCALE),
            ),
            _ => Point2::new(0.0, 0.0),
        };

        // Rect.top_left location
        let bound_top_left_position = position.clone();

        // Rect.bottom_right location
        let bound_bottom_right_position = Point2::new(
            bound_top_left_position.x + (TILE_SIZE * SCALE),
            bound_top_left_position.y + (TILE_SIZE * SCALE),
        );

        // Attack Rect.top_left location
        let attack_top_left_position = Point2::new(
            bound_top_left_position.x + (20.0 * SCALE),
            bound_top_left_position.y + (6.0 * SCALE),
        );

        // Attack Rect.bottom_right location
        let attack_bottom_right_position = Point2::new(
            bound_bottom_right_position.x,
            bound_top_left_position.y - (17.0 * SCALE),
        );

        let facing = match id {
            0 => Action::Right,
            1 => Action::Left,
            _ => Action::Right,
        };

        Entity {
            id: id,
            hp: 5,
            dmg: 1,
            entity_actions: EntityActions::new(facing),
            pos: position,
            animator: Animator::new(3, Duration::from_millis(188)),
            vel: Point2::new(0.0, 0.0),
            bound: Rect::new(bound_top_left_position, bound_bottom_right_position),
            attack_bound: Rect::new(attack_top_left_position, attack_bottom_right_position),
            scale: Point2::new(SCALE, SCALE),
        }
    }

    pub fn update(&mut self) -> GameResult {
        // update velocity
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
        // TODO: Add in jumping

        // update position
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;

        self.bound.translate(&self.vel);

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
        if self.entity_actions.moving_left || self.entity_actions.moving_right {
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
            // not moving
            // stances
            if self.entity_actions.blocking {
                draw_param_index = 0;
            } else if self.entity_actions.attacking {
                draw_param_index = 1;
            } else {
                draw_param_index = 2;
            };
        }

        let mut draw_param = entity_drawparams[draw_param_index].dest(self.pos.as_mint_point());
        if self.id == 1 {
            // we are facing the left and thus need to invert the scale
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

pub struct GameMatch {
    pub id: usize,
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
            health_bar_1: hp_bar_1,
            health_bar_2: hp_bar_2,
            entities: entity_vector,
        }
    }

    pub fn update(&mut self) -> GameResult {
        // update entities
        // TODO: collision check for attacking
        for entity in &mut self.entities {
            entity.update().unwrap();
        }

        self.health_bar_1.update(2);
        self.health_bar_2.update(3);


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
