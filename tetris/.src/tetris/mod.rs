mod graphics;
mod matrix;
mod next_queue;
mod player;
mod tetris;
mod tetromino;

use raylib::prelude::{init, Color, RaylibDraw, RaylibDrawHandle, RaylibHandle, RaylibThread};

use self::graphics::{Drawable, Drawer};

pub struct Game {
    engine: RaylibHandle,
    thread_lock: RaylibThread,
}

impl Game {
    pub fn new() -> Game {
        let (engine, thread_lock) = init().vsync().size(640, 480).title("Tetris in Rust").build();
        Game { engine, thread_lock }
    }

    pub fn start(&mut self) {
        while !self.engine.window_should_close() {
            self.render();
        }
    }

    fn render(&mut self) {
        let mut canvas = self.engine.begin_drawing(&self.thread_lock);
        canvas.draw(self);
    }
}

impl Drawable for Game {
    fn draw(&self, canvas: &mut RaylibDrawHandle) {
        canvas.clear_background(Color::WHITE);
    }
}

// impl Display for Tetris {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         write!(
//             f,
//             "{}    {}",
//             self.matrix.clone().solidify(self.falling_tetromino.clone()),
//             self.next_queue
//         )
//     }
// }
