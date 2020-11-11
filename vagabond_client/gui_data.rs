use ggez::graphics;
use ggez::graphics::{Color, DrawParam, Font, Mesh, MeshBuilder, Text, TextFragment};
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};

use serde::{Deserialize, Serialize};

use cgmath::Vector2;

use crate::constants::{MAX_HP, SCALE, SCREEN_WIDTH};
use crate::geometry::{Point2, Rect};

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
        let mut mesh_builder = MeshBuilder::new();

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

    pub fn draw(&self, ctx: &mut Context, font: &Font) -> GameResult {
        let scale = graphics::Scale::uniform(36.0);

        let (timer_text, rect_mesh) =
            create_text_with_background(ctx, self.current.to_string(), font, scale);

        let location = Point2::new(
            (SCREEN_WIDTH / 2.0) - (timer_text.width(ctx) as f32 / 2.0),
            0.0,
        );

        graphics::draw(
            ctx,
            &rect_mesh,
            DrawParam::new().dest(location.as_mint_point()),
        )
        .unwrap();

        graphics::draw(
            ctx,
            &timer_text,
            DrawParam::new().dest(location.as_mint_point()),
        )
        .unwrap();

        Ok(())
    }
}

pub struct Button {
    bounds: Rect,
    hovered_color: Color,
    unhovered_color: Color,
    clicked_color: Color,
    mouse_hover: bool,
    mouse_clicked: bool,
    button_text: Text,
    button_text_position: Point2,
    text_width: f32,
    text_height: f32,
    button_mesh_outline: Mesh, // will be used to create trim around button
    button_mesh_filled: Mesh,
    pub visible: bool,
}
impl Button {
    pub fn new(
        ctx: &mut Context,
        text: String,
        font: &Font,
        text_scale: graphics::Scale,
        position: Point2,
        visible: bool,
    ) -> Button {
        // initialize colors
        let unhovered_color = Color::new(1.0, 1.0, 1.0, 1.0);
        let hovered_color = Color::new(0.75, 0.75, 0.75, 1.0);
        let clicked_color = Color::new(0.5, 0.5, 0.5, 1.0);

        // set Text
        let button_text = create_text(text, font, text_scale);

        // set position of text
        let button_text_pos = Point2::new(position.x + 5.0, position.y + 5.0);

        // get width and height of text
        let text_width = button_text.width(ctx) as f32;
        let text_height = button_text.height(ctx) as f32;

        // create Rect
        let top_left = position;
        let bottom_right = Point2::new(
            top_left.x + text_width + 10.0,
            top_left.y + text_height + 10.0,
        );
        let rect = Rect::new(top_left, bottom_right);

        // create meshes
        let rect_mesh_outline = create_rect_mesh(
            ctx,
            &rect,
            graphics::DrawMode::Stroke(graphics::StrokeOptions::default()),
            Color::new(0.0, 0.0, 0.0, 1.0),
        );
        let rect_mesh_filled = create_rect_mesh(
            ctx,
            &rect,
            graphics::DrawMode::Fill(graphics::FillOptions::default()),
            unhovered_color.clone(),
        );

        Button {
            bounds: rect,
            hovered_color: hovered_color,
            unhovered_color: unhovered_color,
            clicked_color: clicked_color,
            mouse_hover: false,
            mouse_clicked: false,
            button_text: button_text,
            button_text_position: button_text_pos,
            text_width: text_width,
            text_height: text_height,
            button_mesh_outline: rect_mesh_outline,
            button_mesh_filled: rect_mesh_filled,
            visible: visible,
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        if self.visible == false {
            // we don't want to draw anything
            return Ok(());
        }

        if self.mouse_hover == true && self.mouse_clicked == false {
            // hovering but was not clicked
            graphics::draw(
                ctx,
                &self.button_mesh_filled,
                DrawParam::new()
                    .color(self.hovered_color.clone())
                    .dest(self.bounds.top_left.as_mint_point()),
            )
            .unwrap();
        } else if self.mouse_clicked == true {
            // clicked
            graphics::draw(
                ctx,
                &self.button_mesh_filled,
                DrawParam::new()
                    .color(self.clicked_color.clone())
                    .dest(self.bounds.top_left.as_mint_point()),
            )
            .unwrap();
        } else {
            // not hovering over
            graphics::draw(
                ctx,
                &self.button_mesh_filled,
                DrawParam::new()
                    .color(self.unhovered_color.clone())
                    .dest(self.bounds.top_left.as_mint_point()),
            )
            .unwrap();
        }

        // outline the button
        graphics::draw(
            ctx,
            &self.button_mesh_outline,
            DrawParam::new().dest(self.bounds.top_left.as_mint_point()),
        )
        .unwrap();

        // draw text
        graphics::draw(
            ctx,
            &self.button_text,
            DrawParam::new()
                .dest(self.button_text_position.as_mint_point())
                .color(Color::new(0.0, 0.0, 0.0, 1.0)),
        )
        .unwrap();

        Ok(())
    }

    pub fn mouse_clicked(&mut self) -> bool {
        self.mouse_clicked
    }
    pub fn mouse_motion_event(&mut self, mouse_position: &Point2) {
        if self.visible == false {
            return ();
        }
        if self.bounds.check_bounds_point(mouse_position) == true {
            self.mouse_hover = true;
        } else {
            self.mouse_hover = false;
        }
    }

    pub fn mouse_button_down_event(&mut self, mouse_button: &MouseButton, mouse_position: &Point2) {
        if self.visible == false {
            return ();
        }
        match mouse_button {
            MouseButton::Left => {
                if self.bounds.check_bounds_point(mouse_position) == true {
                    self.mouse_clicked = true;
                }
            }
            _ => (),
        }
    }

    pub fn mouse_button_up_event(&mut self, mouse_button: &MouseButton, mouse_position: &Point2) {
        if self.visible == false {
            return ();
        }
        match mouse_button {
            MouseButton::Left => {
                if self.bounds.check_bounds_point(mouse_position) == true {
                    self.mouse_clicked = false;
                }
            }
            _ => (),
        }
    }

    pub fn change_location(&mut self, position: Point2) {
        self.bounds.top_left = position;
        self.bounds.bottom_right = Point2::new(
            self.bounds.top_left.x + self.text_width + 10.0,
            self.bounds.top_left.y + self.text_height + 10.0,
        );
        self.button_text_position = Point2::new(position.x + 5.0, position.y + 5.0);
    }

    pub fn get_text_width(&self) -> f32 {
        self.text_width
    }

    pub fn get_text_height(&self) -> f32 {
        self.text_height
    }
}

pub fn create_text_with_background(
    ctx: &mut Context,
    text: String,
    font: &Font,
    scale: graphics::Scale,
) -> (Text, Mesh) {
    let text_graphic = create_text(text, font, scale);

    let text_width = text_graphic.width(ctx) as f32;
    let text_height = text_graphic.height(ctx) as f32;

    let top_left = Point2::new(0.0, 0.0);
    let bottom_right = Point2::new(text_width, text_height);
    let text_background_rect = Rect::new(top_left, bottom_right); // you will have to change the destination after it's creation

    let rect_mesh = create_rect_mesh(
        ctx,
        &text_background_rect,
        graphics::DrawMode::Fill(graphics::FillOptions::default()),
        Color::new(0.25, 0.25, 0.25, 0.75),
    );

    (text_graphic, rect_mesh)
}

fn create_text(text: String, font: &Font, scale: graphics::Scale) -> Text {
    let text_fragment = TextFragment::new(text.to_string())
        .scale(scale)
        .font(font.clone());

    let text_graphic = Text::new(text_fragment);
    text_graphic
}

fn create_rect_mesh(
    ctx: &mut Context,
    rectangle_bounds: &Rect,
    draw_mode: graphics::DrawMode,
    color: Color,
) -> Mesh {
    let mut mesh_builder = MeshBuilder::new();
    let rect = graphics::Rect::new(
        rectangle_bounds.top_left.x,
        rectangle_bounds.top_left.y,
        rectangle_bounds.bottom_right.x - rectangle_bounds.top_left.x,
        rectangle_bounds.bottom_right.y - rectangle_bounds.top_left.y,
    );

    mesh_builder
        .rectangle(draw_mode, rect, color)
        .build(ctx)
        .unwrap()
}
