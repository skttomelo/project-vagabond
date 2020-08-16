use ggez;
use ggez::graphics;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler, KeyCode, KeyMods};

mod entities;
use entities::{Entity, Player};

struct MainState {
    player: Player,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let p = Player::new(Point2::<f32>::new(0.0,0.0), Point2::<f32>::new(0.0,0.0), 30.0);
        let s = MainState { player: p };
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        self.player.update(_ctx)?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        self.player.draw(ctx)?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        self.player.key_down_event(keycode, _keymods, _repeat);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        self.player.key_up_event(keycode, _keymods);
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Vagabond Client", "Trevor Crow");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}