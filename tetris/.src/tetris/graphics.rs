use raylib::prelude::RaylibDrawHandle;

pub trait Drawable {
    fn draw(&self, canvas: &mut RaylibDrawHandle);
}

pub trait Drawer {
    fn draw(&mut self, drawable: &impl Drawable);
}

impl Drawer for RaylibDrawHandle<'_> {
    fn draw(&mut self, drawable: &impl Drawable) {
        drawable.draw(self);
    }
}
