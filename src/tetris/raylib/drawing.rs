use super::raylib_c::{
    BeginDrawing, ClearBackground, DrawCircleV, DrawFPS, DrawRectangleV, DrawTextPro, EndDrawing, Font, GetFontDefault,
    Vector2,
};
use super::shapes::{Circle, Rectangle};
use super::window::Window;

use std::ffi::CString;

pub(in super::super) use super::raylib_c::Color;

pub(in super::super) trait Drawable<'a> {
    fn draw(&self, canvas: Canvas<'a>) -> Canvas<'a>;
}

impl Window {
    pub(in super::super) fn canvas(&mut self) -> Canvas {
        Canvas::new(self)
    }
}

pub(in super::super) struct Canvas<'a> {
    pub(in super::super) window: &'a mut Window,
}

impl<'a> Canvas<'a> {
    fn new(window: &'a mut Window) -> Self {
        unsafe {
            BeginDrawing();
        }

        Canvas { window }
    }

    pub(in super::super) fn draw(self, drawable: &impl Drawable<'a>) -> Self {
        drawable.draw(self)
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
pub(in super::super) struct Text {
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
    pub(in super::super) fn new(text: impl Into<String>, position: Vector2, font_size: f32, color: Color) -> Self {
        const DEFAULT_FONT_SIZE: f32 = 10.0;

        Text {
            text: text.into(),
            position,
            font_size,
            color,
            spacing: font_size / DEFAULT_FONT_SIZE,
            ..Default::default()
        }
    }

    pub(in super::super) fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub(in super::super) fn position(mut self, position: Vector2) -> Self {
        self.position = position;
        self
    }

    pub(in super::super) fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub(in super::super) fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub(in super::super) fn font(mut self, font: Font) -> Self {
        self.font = font;
        self
    }

    pub(in super::super) fn origin(mut self, origin: Vector2) -> Self {
        self.origin = origin;
        self
    }

    pub(in super::super) fn rotation(mut self, rotation: Rotation) -> Self {
        self.rotation = rotation;
        self
    }

    pub(in super::super) fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

impl<'a> Drawable<'a> for Text {
    fn draw(&self, canvas: Canvas<'a>) -> Canvas<'a> {
        canvas.draw_text(
            &self.text,
            self.position,
            self.font_size,
            self.color,
            self.font,
            self.origin,
            self.rotation,
            self.spacing,
        )
    }
}

pub(in super::super) struct Background {
    pub(in super::super) color: Color,
}

impl<'a> Drawable<'a> for Background {
    fn draw(&self, canvas: Canvas<'a>) -> Canvas<'a> {
        canvas.clear_background(self.color)
    }
}

pub(in super::super) struct RectangleGraphic {
    pub(in super::super) rectangle: Rectangle,
    pub(in super::super) position: Vector2,
    pub(in super::super) color: Color,
}

impl<'a, 'b> Drawable<'a> for RectangleGraphic {
    fn draw(&self, canvas: Canvas<'a>) -> Canvas<'a> {
        canvas.draw_rectangle(self.position, self.rectangle.size, self.color)
    }
}

pub(in super::super) struct CircleGraphic<'a> {
    pub(in super::super) circle: &'a Circle,
    pub(in super::super) center: Vector2,
    pub(in super::super) color: Color,
}

impl<'a, 'b> Drawable<'a> for CircleGraphic<'b> {
    fn draw(&self, canvas: Canvas<'a>) -> Canvas<'a> {
        canvas.draw_circle(self.center, self.circle.radius, self.color)
    }
}

pub(in super::super) struct FPSGraphic {
    pub(in super::super) position: Vector2,
}

impl<'a> Drawable<'a> for FPSGraphic {
    fn draw(&self, canvas: Canvas<'a>) -> Canvas<'a> {
        canvas.draw_fps(self.position)
    }
}

impl<'a> Canvas<'a> {
    fn draw_rectangle(self, position: Vector2, size: Vector2, color: Color) -> Self {
        unsafe {
            DrawRectangleV(
                Vector2 {
                    x: position.x,
                    y: position.y,
                },
                size,
                color,
            )
        }
        self
    }

    fn draw_circle(self, center: Vector2, radius: f32, color: Color) -> Self {
        unsafe { DrawCircleV(center, radius, color) }
        self
    }

    fn clear_background(self, color: Color) -> Self {
        unsafe { ClearBackground(color) }
        self
    }

    fn draw_fps(self, position: Vector2) -> Self {
        unsafe { DrawFPS(position.x as i32, position.y as i32) }
        self
    }

    fn draw_text(
        self,
        text: &str,
        position: Vector2,
        font_size: f32,
        color: Color,
        font: Font,
        origin: Vector2,
        rotation: Rotation,
        spacing: f32,
    ) -> Self {
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
            );
        }
        self
    }
}

#[derive(Clone, Copy)]
pub(in super::super) struct Rotation(f32);

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
    pub(in super::super) const LIGHT_GRAY: Color = Color {
        r: 200,
        g: 200,
        b: 200,
        a: 255,
    };
    pub(in super::super) const GRAY: Color = Color {
        r: 130,
        g: 130,
        b: 130,
        a: 255,
    };
    pub(in super::super) const DARK_GRAY: Color = Color {
        r: 80,
        g: 80,
        b: 80,
        a: 255,
    };
    pub(in super::super) const YELLOW: Color = Color {
        r: 253,
        g: 249,
        b: 0,
        a: 255,
    };
    pub(in super::super) const GOLD: Color = Color {
        r: 255,
        g: 203,
        b: 0,
        a: 255,
    };
    pub(in super::super) const ORANGE: Color = Color {
        r: 255,
        g: 161,
        b: 0,
        a: 255,
    };
    pub(in super::super) const PINK: Color = Color {
        r: 255,
        g: 109,
        b: 194,
        a: 255,
    };
    pub(in super::super) const RED: Color = Color {
        r: 230,
        g: 41,
        b: 55,
        a: 255,
    };
    pub(in super::super) const MAROON: Color = Color {
        r: 190,
        g: 33,
        b: 55,
        a: 255,
    };
    pub(in super::super) const GREEN: Color = Color {
        r: 0,
        g: 228,
        b: 48,
        a: 255,
    };
    pub(in super::super) const LIME: Color = Color {
        r: 0,
        g: 158,
        b: 47,
        a: 255,
    };
    pub(in super::super) const DARK_GREEN: Color = Color {
        r: 0,
        g: 117,
        b: 44,
        a: 255,
    };
    pub(in super::super) const SKY_BLUE: Color = Color {
        r: 102,
        g: 191,
        b: 255,
        a: 255,
    };
    pub(in super::super) const BLUE: Color = Color {
        r: 0,
        g: 121,
        b: 241,
        a: 255,
    };
    pub(in super::super) const DARKBLUE: Color = Color {
        r: 0,
        g: 82,
        b: 172,
        a: 255,
    };
    pub(in super::super) const PURPLE: Color = Color {
        r: 200,
        g: 122,
        b: 255,
        a: 255,
    };
    pub(in super::super) const VIOLET: Color = Color {
        r: 135,
        g: 60,
        b: 190,
        a: 255,
    };
    pub(in super::super) const DARK_PURPLE: Color = Color {
        r: 112,
        g: 31,
        b: 126,
        a: 255,
    };
    pub(in super::super) const BEIGE: Color = Color {
        r: 211,
        g: 176,
        b: 131,
        a: 255,
    };
    pub(in super::super) const BROWN: Color = Color {
        r: 127,
        g: 106,
        b: 79,
        a: 255,
    };
    pub(in super::super) const DARK_BROWN: Color = Color {
        r: 76,
        g: 63,
        b: 47,
        a: 255,
    };
    pub(in super::super) const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub(in super::super) const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub(in super::super) const BLANK: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    pub(in super::super) const MAGENTA: Color = Color {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
    };
    pub(in super::super) const RAY_WHITE: Color = Color {
        r: 245,
        g: 245,
        b: 245,
        a: 255,
    };
}

impl Default for Color {
    fn default() -> Self {
        Color::BLACK
    }
}
