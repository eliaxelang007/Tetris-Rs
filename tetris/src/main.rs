use raylib::{
    drawing::{Background, CircleGraphic, Color, Text},
    input::KeyboardKey,
    shapes::{Circle, Vector2},
    RaylibBuilder,
};

fn main() {
    const WINDOW_WIDTH: u16 = 800;
    const WINDOW_HEIGHT: u16 = 450;

    let mut raylib = RaylibBuilder::new("Hello, world!", WINDOW_WIDTH, WINDOW_HEIGHT)
        .vsync()
        .build();

    let mut ball_position = Vector2 {
        x: (WINDOW_WIDTH as f32) / 2.0,
        y: (WINDOW_HEIGHT as f32) / 2.0,
    };

    const SPEED: f32 = 100.0;

    while !raylib.window_should_close() {
        let frame_time = raylib.get_frame_time().as_secs_f32();

        use KeyboardKey::*;

        let x_input = match (raylib.is_key_down(KEY_RIGHT), raylib.is_key_down(KEY_LEFT)) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            (_, _) => 0.0,
        };

        ball_position.x += frame_time * SPEED * x_input;

        let y_input = match (raylib.is_key_down(KEY_UP), raylib.is_key_down(KEY_DOWN)) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            (_, _) => 0.0,
        };

        ball_position.y += frame_time * SPEED * y_input;

        let mut canvas = raylib.begin_drawing();

        canvas.draw(Background {
            color: Color::RAY_WHITE,
        });

        canvas.draw(Text::new(
            "move the ball with arrow keys".to_string(),
            Vector2 { x: 10.0, y: 10.0 },
            20.0,
            Color::DARK_GRAY,
        ));

        canvas.draw(CircleGraphic {
            circle: Circle {
                center: ball_position,
                radius: 50.0,
            },
            color: Color::MAROON,
        });
    }
}
