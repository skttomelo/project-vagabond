use ggez;
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::graphics::Image;
use ggez::conf::{FullscreenType, WindowMode};
use ggez::{Context, GameResult};

use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use std::net::TcpStream;

mod game_data;
use game_data::{Entity, GameMatch};

struct MainState {
    game_match: GameMatch,
    entity_assets: Vec<Image>,
    background_asset: Image,
    server: Option<TcpStream>
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let gm = GameMatch::new();
        //TODO: factor out long expression here with some iteration through the directory for images
        // load images in
        let entity_assets = vec![Image::new(ctx, Path::new("Samurai/samurai_blocking_stance.png")).unwrap(),
                                 Image::new(ctx, Path::new("Samurai/samurai_fighting_stance.png")).unwrap(),
                                 Image::new(ctx, Path::new("Samurai/samurai_idle_stance.png")).unwrap(),
                                 Image::new(ctx, Path::new("Samurai/samurai_walk_1.png")).unwrap(),
                                 Image::new(ctx, Path::new("Samurai/samurai_walk_2.png")).unwrap(),
                                 Image::new(ctx, Path::new("Samurai/samurai_walk_3.png")).unwrap(),
                                 Image::new(ctx, Path::new("Samurai/samurai_walk_4.png")).unwrap(),
                                 Image::new(ctx, Path::new("Samurai/samurai_walk_5.png")).unwrap()];
        let background_asset = Image::new(ctx, Path::new("Backgrounds/Dojo.png")).unwrap();
        let s = MainState { game_match: gm, entity_assets: entity_assets, background_asset: background_asset, server: None};
        Ok(s)
    }

    fn load_images(directory: &Path) -> Option<(Vec<Image>, Image)> {
        if directory.is_dir() == false {
            return None;
        }
        directory.join("Samurai");
        let directories = fs::read_dir(&directory);

        let background: Image;

        let samurai: Vec<Image> = directories.map(|entry|{
            let entry = entry.unwrap();
        }).collect::<Vec<Image>>();

        Some((samurai, background))
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // self.player.update(_ctx)?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // self.player.draw(ctx)?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        // self.player.key_down_event(keycode, _keymods, _repeat);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        // self.player.key_up_event(keycode, _keymods);
    }
}

pub fn main() -> GameResult {
    // window
    let window = WindowMode {
        width: 800.0,
        height: 600.0,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: false,
    };

    let cb = ggez::ContextBuilder::new("Vagabond Client", "Trevor Crow");
    
    // build and split context builder with window configuration
    let (ctx, event_loop) = &mut cb.window_mode(window).build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
