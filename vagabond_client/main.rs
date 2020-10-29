use ggez;
use ggez::conf::{FullscreenType, WindowMode};
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::graphics::{DrawParam, FilterMode, Font, Image, Rect};
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};

use cgmath::Vector2;

use serde_json;

use std::env;
use std::net::{TcpStream}; // Shutdown
use std::path::{Path, PathBuf};
use std::io::{Read, Write};
use std::str::from_utf8;

mod animate;
mod constants;
mod entity_data;
mod game_data;
mod geometry;
mod gui_data;
mod server_data;

use constants::{SCALE, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};
use game_data::{ControlledActor, GameMatch};
use server_data::ServerGameMatch;

/*************************************************************
 *  TODO: Place all images into a spritesheet and subdivide  *
 *  images from the sheet so consistent animations can be    *
 *  achieved.                                                *
 *************************************************************/

struct MainState {
    game_match: GameMatch,
    entity_spritesheet: Image,
    entity_drawparams: Vec<DrawParam>,
    background_assets: Vec<Image>,
    font: Font,
    server: TcpStream,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut server = TcpStream::connect("127.0.0.1:1337").unwrap();

        let mut data = [0u8; 4096];

        let id: usize;
        let mut string_data: String;

        // first acquire id
        server.read(&mut data).unwrap();
        string_data = String::from(from_utf8(&data).unwrap());
        string_data = String::from(string_data.trim_matches(char::from(0)));
        id = string_data.parse().unwrap(); // because id's type is declared earlier we do not need to do `parse::<u8>()`
        println!("{}", id);
        let gm = GameMatch::new(id);

        let (entity_spritesheet, entity_drawparams, background_assets) =
            MainState::load_images(ctx);

        let font = Font::new(ctx, "/Fonts/PressStart2P-vaV7.ttf").unwrap();

        let s = MainState {
            game_match: gm,
            entity_spritesheet: entity_spritesheet,
            entity_drawparams: entity_drawparams,
            background_assets: background_assets,
            font: font,
            server: server,
        };
        Ok(s)
    }

    fn load_images(ctx: &mut Context) -> (Image, Vec<DrawParam>, Vec<Image>) {
        // get path to Background and Samurai directories
        let mut background_directory = Path::new("/Backgrounds/dojo.png");

        let samurai_directory = Path::new("/Samurai/samurai_spritesheet.png");

        // load background image
        let mut background_images: Vec<Image> = Vec::new();

        let mut bg = Image::new(ctx, &background_directory).unwrap();
        bg.set_filter(FilterMode::Nearest); // remove blur
        background_images.push(bg);

        background_directory = Path::new("/Backgrounds/dojo_inside.png");
        let mut bg = Image::new(ctx, &background_directory).unwrap();
        bg.set_filter(FilterMode::Nearest); //remove blur
        background_images.push(bg);

        background_images.push(Image::new(ctx, background_directory).unwrap());

        // load samurai images
        let mut samurai_image = Image::new(ctx, samurai_directory).unwrap();
        // Nearest removes blur as well as fixes the bleed over with the spritesheet
        samurai_image.set_filter(FilterMode::Nearest);

        let mut samurai_drawparams: Vec<DrawParam> = Vec::new();

        let mut counter = 0;
        let image_width = samurai_image.width();
        let scale = Vector2::<f32>::new(SCALE, SCALE);
        let width = TILE_SIZE / image_width as f32;
        let increment = image_width / 8; // we have six frames inside of the spritesheet
        while counter < image_width {
            // location in spritesheet
            let x = counter as f32 / image_width as f32;
            let frame = Rect::new(x, 0.0, width, 1.0);
            // we add scaling to the images so we don't deal with the calculations later
            let draw_param = DrawParam::new().src(frame).scale(scale);

            samurai_drawparams.push(draw_param);
            //increment
            counter += increment;
        }

        (samurai_image, samurai_drawparams, background_images)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // create the ServerGameMatch that we will serialize
        let server_game_match = ServerGameMatch::from_game_match(&self.game_match);

        // serialize the data
        let serialized_data = serde_json::to_string(&server_game_match).expect("Unable to serialize ServerGameMatch");
        println!("{}", &serialized_data);

        // convert data to byte array and send to server
        let data_as_bytes = serialized_data.into_bytes();

        // Send serialized byte data to server
        self.server.write_all(&data_as_bytes).expect("Could not write bytes to stream");
        self.server.flush().expect("Could not flush stream");


        // code for receiving the data from the server goes here
        let mut data = [0u8; 1024];

        self.server.read(&mut data).unwrap();
        let mut string_data = String::from(from_utf8(&data).unwrap());
        string_data = string_data.trim_matches(char::from(0)).to_owned();

        // deserialized data code goes here
        let server_match: ServerGameMatch = serde_json::from_str(&string_data).unwrap();
        self.game_match.update_from_server_game_match(&server_match);

        // update the match on client end
        self.game_match.update().unwrap();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // draw background
        graphics::draw(
            ctx,
            &self.background_assets[0],
            graphics::DrawParam::new()
                .dest(Point2::<f32>::new(0.0, 0.0))
                .scale(Vector2::<f32>::new(4.0, 4.0)),
        )
        .expect("Draw call failed");

        // draw everything else
        self.game_match
            .draw(
                ctx,
                &self.entity_spritesheet,
                &self.entity_drawparams,
                &self.font,
            )
            .expect("Draw call for GameMatch failed");

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
        self.game_match.key_down_event(keycode, _keymods, _repeat);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        self.game_match.key_up_event(keycode, keymods);
    }
}

pub fn main() -> GameResult {
    // window
    let window = WindowMode {
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
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
