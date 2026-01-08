pub trait Drawable {
    fn draw(&self, canvas: Canvas) -> Canvas;
}

pub struct Canvas {}

impl Canvas {
    pub fn draw(self, drawable: &impl Drawable) -> Self {
        drawable.draw(self)
    }
}

pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

pub struct RectangleGraphic {
    pub rectangle: Rectangle,
    pub position: Vector2,
    pub color: Color,
}

impl Drawable for RectangleGraphic {
    fn draw(&self, canvas: Canvas) -> Canvas {
        canvas.draw_rectangle(self.position, self.rectangle.size, self.color)
    }
}
