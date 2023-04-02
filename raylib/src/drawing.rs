use super::raylib::Raylib;
use super::raylib_c::{
    BeginDrawing, ClearBackground, DrawCircleV, DrawTextPro, EndDrawing, Font, GetFontDefault, Vector2,
};
use super::shapes::Circle;

use std::ffi::CString;

pub use super::raylib_c::Color;

pub struct Canvas<'a> {
    _raylib: &'a mut Raylib,
}

impl<'a> Canvas<'a> {
    fn new(raylib: &'a mut Raylib) -> Self {
        unsafe {
            BeginDrawing();
        }

        Canvas { _raylib: raylib }
    }

    pub fn draw(&mut self, drawable: impl Drawable + 'a) {
        drawable.draw(self)
    }

    pub(super) fn draw_circle(&self, center: Vector2, radius: f32, color: Color) {
        unsafe { DrawCircleV(center, radius, color) }
    }

    fn clear_background(&self, color: Color) {
        unsafe { ClearBackground(color) }
    }

    fn draw_text(
        &self,
        text: String,
        position: Vector2,
        font_size: f32,
        color: Color,
        font: Font,
        origin: Vector2,
        rotation: Rotation,
        spacing: f32,
    ) {
        let text = CString::new(text).unwrap();
        unsafe {
            DrawTextPro(
                font,
                text.as_ptr(),
                position,
                origin,
                rotation.into(),
                font_size,
                spacing,
                color,
            )
        }
    }
}

impl<'a> Drop for Canvas<'a> {
    fn drop(&mut self) {
        unsafe {
            EndDrawing();
        }
    }
}

#[derive(Default)]
pub struct Text {
    text: String,
    position: Vector2,
    font_size: f32,
    color: Color,
    font: Font,
    origin: Vector2,
    rotation: Rotation,
    spacing: f32,
}

impl Text {
    pub fn new(text: String, position: Vector2, font_size: f32, color: Color) -> Self {
        const DEFAULT_FONT_SIZE: f32 = 10.0;

        Text {
            text,
            position,
            font_size,
            color,
            spacing: font_size / DEFAULT_FONT_SIZE,
            ..Default::default()
        }
    }

    pub fn text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    pub fn position(mut self, position: Vector2) -> Self {
        self.position = position;
        self
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn font(mut self, font: Font) -> Self {
        self.font = font;
        self
    }

    pub fn origin(mut self, origin: Vector2) -> Self {
        self.origin = origin;
        self
    }

    pub fn rotation(mut self, rotation: Rotation) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

pub struct Background {
    pub color: Color,
}

impl Drawable for Text {
    fn draw(&self, canvas: &mut Canvas) {
        canvas.draw_text(
            self.text.clone(),
            self.position,
            self.font_size,
            self.color,
            self.font,
            self.origin,
            self.rotation,
            self.spacing,
        );
    }
}

impl Drawable for Background {
    fn draw(&self, canvas: &mut Canvas) {
        canvas.clear_background(self.color)
    }
}

pub struct CircleGraphic {
    pub circle: Circle,
    pub color: Color,
}

impl Drawable for CircleGraphic {
    fn draw(&self, canvas: &mut crate::drawing::Canvas) {
        canvas.draw_circle(self.circle.center, self.circle.radius, self.color)
    }
}

pub trait Drawable {
    fn draw(&self, canvas: &mut Canvas);
}

impl Raylib {
    pub fn begin_drawing(&mut self) -> Canvas {
        Canvas::new(self)
    }
}

#[derive(Clone, Copy)]
pub struct Rotation(f32);

impl From<Rotation> for f32 {
    fn from(rotation: Rotation) -> Self {
        rotation.0
    }
}

impl Default for Rotation {
    fn default() -> Self {
        Rotation(0.0)
    }
}

impl Default for Font {
    fn default() -> Self {
        unsafe { GetFontDefault() }
    }
}

impl Color {
    pub const LIGHT_GRAY: Color = Color {
        r: 200,
        g: 200,
        b: 200,
        a: 255,
    };
    pub const GRAY: Color = Color {
        r: 130,
        g: 130,
        b: 130,
        a: 255,
    };
    pub const DARK_GRAY: Color = Color {
        r: 80,
        g: 80,
        b: 80,
        a: 255,
    };
    pub const YELLOW: Color = Color {
        r: 253,
        g: 249,
        b: 0,
        a: 255,
    };
    pub const GOLD: Color = Color {
        r: 255,
        g: 203,
        b: 0,
        a: 255,
    };
    pub const ORANGE: Color = Color {
        r: 255,
        g: 161,
        b: 0,
        a: 255,
    };
    pub const PINK: Color = Color {
        r: 255,
        g: 109,
        b: 194,
        a: 255,
    };
    pub const RED: Color = Color {
        r: 230,
        g: 41,
        b: 55,
        a: 255,
    };
    pub const MAROON: Color = Color {
        r: 190,
        g: 33,
        b: 55,
        a: 255,
    };
    pub const GREEN: Color = Color {
        r: 0,
        g: 228,
        b: 48,
        a: 255,
    };
    pub const LIME: Color = Color {
        r: 0,
        g: 158,
        b: 47,
        a: 255,
    };
    pub const DARK_GREEN: Color = Color {
        r: 0,
        g: 117,
        b: 44,
        a: 255,
    };
    pub const SKY_BLUE: Color = Color {
        r: 102,
        g: 191,
        b: 255,
        a: 255,
    };
    pub const BLUE: Color = Color {
        r: 0,
        g: 121,
        b: 241,
        a: 255,
    };
    pub const DARKBLUE: Color = Color {
        r: 0,
        g: 82,
        b: 172,
        a: 255,
    };
    pub const PURPLE: Color = Color {
        r: 200,
        g: 122,
        b: 255,
        a: 255,
    };
    pub const VIOLET: Color = Color {
        r: 135,
        g: 60,
        b: 190,
        a: 255,
    };
    pub const DARK_PURPLE: Color = Color {
        r: 112,
        g: 31,
        b: 126,
        a: 255,
    };
    pub const BEIGE: Color = Color {
        r: 211,
        g: 176,
        b: 131,
        a: 255,
    };
    pub const BROWN: Color = Color {
        r: 127,
        g: 106,
        b: 79,
        a: 255,
    };
    pub const DARK_BROWN: Color = Color {
        r: 76,
        g: 63,
        b: 47,
        a: 255,
    };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const BLANK: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    pub const MAGENTA: Color = Color {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const RAY_WHITE: Color = Color {
        r: 245,
        g: 245,
        b: 245,
        a: 255,
    };
}

impl Default for Color {
    fn default() -> Self {
        Color::DARK_GRAY
    }
}
