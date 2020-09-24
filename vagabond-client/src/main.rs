use ggez;
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::filesystem;
use ggez::graphics::Image;
use ggez::conf::{FullscreenType, WindowMode};
use ggez::{Context, GameResult};

use std::env;
use std::path::{Path, PathBuf};

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
        // let resource_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("resources");
        // println!("{:?}", &resource_path);

        let (entity_assets, background_asset) = match MainState::load_images(ctx){
            Some((entity_assets, background_asset)) => (entity_assets, background_asset),
            None => panic!("directory does not exist!")
        };
        let s = MainState { game_match: gm, entity_assets: entity_assets, background_asset: background_asset, server: None};
        Ok(s)
    }

    fn load_images(ctx: &mut Context) -> Option<(Vec<Image>, Image)> {
        // get abs path to  Background and Samurai directories
        let background_directory = Path::new("/Backgrounds/dojo.png");
        let samurai_directory = {
            let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
            path.push("resources");
            path.push("Samurai");
            path
        };

        // load background image
        let background_image = Image::new(ctx, background_directory).unwrap();

        // load samurai images
        // let directories = fs::read_dir(&abs_samurai_directory).unwrap();

        let mut samurai_images: Vec<Image> = Vec::new();
        
        for entry in samurai_directory.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = Path::new("/Samurai").join(entry.file_name());

            samurai_images.push(Image::new(ctx, path).unwrap());
        }

        Some((samurai_images, background_image))
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

    let mut cb = ggez::ContextBuilder::new("Vagabond Client", "Trevor Crow");

    // get and add resource path
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        cb = cb.add_resource_path(path);
    }
    
    // build and split context builder with window configuration
    let (ctx, event_loop) = &mut cb.window_mode(window).build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
