use ggez;
use ggez::graphics;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use ggez::event::{KeyCode, KeyMods};

pub enum State{}

pub trait Entity {
    fn update(&mut self, _ctx: &mut Context) -> GameResult;
    fn draw(&mut self, ctx: &mut Context) -> GameResult;
    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool);
    fn key_up_event(&mut self, keycode: KeyCode, _keymods: KeyMods);
}

pub struct Player {
    pos: Point2<f32>,
    vel: Point2<f32>,
    size: f32
}

impl Player {
    pub fn new(pos: Point2<f32>, vel: Point2<f32>, size: f32) -> Player {
        Player{
            pos,
            vel,
            size
        }
    }
}

impl Entity for Player {
    fn update(&mut self, _ctx: &mut Context) -> GameResult{
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;

        // it worked
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.pos,
            self.size,
            1.0,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &circle, (self.pos,))?;

        // it worked
        Ok(())
    }

    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::W => self.vel.y = -1.0, //negative y vel makes things move up
            KeyCode::S => self.vel.y = 1.0,
            KeyCode::A => self.vel.x = -1.0,
            KeyCode::D => self.vel.x = 1.0,
            _ => ()
        }
    }

    fn key_up_event(&mut self, keycode: KeyCode, _keymods: KeyMods) {
        match keycode {
            KeyCode::W | KeyCode::S => self.vel.y = 0.0,
            KeyCode::A | KeyCode::D => self.vel.x = 0.0,
            _ => ()
        }
    }
}