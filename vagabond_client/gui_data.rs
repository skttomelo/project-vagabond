use ggez::graphics;
use ggez::graphics::{Color, DrawParam, Font, Text, TextFragment, Mesh};
use ggez::{Context, GameResult};

use serde::{Deserialize, Serialize};

use cgmath::Vector2;

use crate::constants::{MAX_HP, SCALE, SCREEN_WIDTH};

use crate::geometry::Point2;

pub struct HealthBar {
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
            _ => 10.0 * SCALE * 4.0,
        };
        let height = 3.0 * SCALE * 2.0;
        let f_color = Color::new(1.0, 0.0, 0.0, 1.0);
        let b_color = Color::new(0.5, 0.5, 0.5, 0.5);

        // id corresponds to the player
        let position = match id {
            1 => Point2::new(SCREEN_WIDTH, 0.0),
            _ => Point2::new(0.0, 0.0),
        };

        let f_rect = graphics::Rect::new(position.x, position.y, width, height);
        let b_rect = graphics::Rect::new(position.x, position.y, width, height);

        let draw_param = DrawParam::new().src(f_rect.clone());

        HealthBar {
            id: id,
            max_hp: MAX_HP,
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
            self.draw_param = self.draw_param.scale(Vector2::<f32>::new(-1.0, 1.0))
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let mut mesh_builder = graphics::MeshBuilder::new();

        // draw background first
        let b_mesh = mesh_builder
            .rectangle(
                graphics::DrawMode::Fill(graphics::FillOptions::default()),
                self.background_rectangle,
                self.background_color,
            )
            .build(ctx)
            .unwrap();

        // draw foreground
        let f_mesh = mesh_builder
            .rectangle(
                graphics::DrawMode::Fill(graphics::FillOptions::default()),
                self.foreground_rectangle,
                self.foreground_color,
            )
            .build(ctx)
            .unwrap();

        graphics::draw(ctx, &b_mesh, self.draw_param.color(self.background_color)).unwrap();
        graphics::draw(ctx, &f_mesh, self.draw_param.color(self.foreground_color)).unwrap();

        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Clock {
    current: u16,
}
impl Clock {
    pub fn new() -> Clock {
        Clock { current: 0 }
    }

    // will be used when deserializing data
    #[allow(dead_code)]
    pub fn update(&mut self, time: u16) {
        self.current = time;
    }

    pub fn draw(&self, ctx: &mut Context, font: &Font) -> GameResult {
        let scale = graphics::Scale::uniform(36.0);

        let (timer_text, rect_mesh) = create_text_with_background(ctx, self.current.to_string(), font.clone(), scale);

        let location = Point2::new((SCREEN_WIDTH / 2.0) - (timer_text.width(ctx) as f32 / 2.0), 0.0);

        graphics::draw(ctx, &rect_mesh, DrawParam::new().dest(location.as_mint_point())).unwrap();

        graphics::draw(
            ctx,
            &timer_text,
            DrawParam::new().dest(location.as_mint_point()),
        )
        .unwrap();

        Ok(())
    }
}

pub fn create_text_with_background(ctx: &mut Context, text: String, font: Font, scale: graphics::Scale) -> (Text, Mesh) {
    let text_fragment = TextFragment::new(text.to_string())
            .scale(scale)
            .font(font.clone());

    let text_graphic = Text::new(text_fragment);

    let text_width = text_graphic.width(ctx) as f32;
    let text_height = text_graphic.height(ctx) as f32;

    let mut mesh_builder = graphics::MeshBuilder::new();
    let text_background_rect =
        graphics::Rect::new(0.0, 0.0, text_width, text_height); // you will have to change the destination after it's creation
    let rect_mesh = mesh_builder
        .rectangle(
            graphics::DrawMode::Fill(graphics::FillOptions::default()),
            text_background_rect,
            Color::new(0.25, 0.25, 0.25, 0.75),
        )
        .build(ctx)
        .unwrap();
    
    (text_graphic, rect_mesh)
}
