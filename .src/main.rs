// use raylib::{
//     drawing::{Background, CircleGraphic, Color, Text},
//     input::KeyboardKey,
//     shapes::{Circle, Vector2},
//     RaylibBuilder,
// };

// fn main() {
//     const WINDOW_WIDTH: u16 = 800;
//     const WINDOW_HEIGHT: u16 = 450;

//     let mut raylib = RaylibBuilder::new("Hello, world!", WINDOW_WIDTH, WINDOW_HEIGHT)
//         .vsync()
//         .build();

//     let window = &mut raylib.window;
//     let input = &raylib.input;

//     let mut ball_position = Vector2 {
//         x: (WINDOW_WIDTH as f32) / 2.0,
//         y: (WINDOW_HEIGHT as f32) / 2.0,
//     };

//     const SPEED: f32 = 200.0;

//     while !window.should_close() {
//         let frame_time = window.frame_time().as_secs_f32();

//         use KeyboardKey::*;

//         let x_input = match (input.key_down(KEY_RIGHT), input.key_down(KEY_LEFT)) {
//             (true, false) => 1.0,
//             (false, true) => -1.0,
//             (_, _) => 0.0,
//         };

//         ball_position.x += frame_time * SPEED * x_input;

//         let y_input = match (input.key_down(KEY_UP), input.key_down(KEY_DOWN)) {
//             (true, false) => -1.0,
//             (false, true) => 1.0,
//             (_, _) => 0.0,
//         };

//         ball_position.y += frame_time * SPEED * y_input;

//         window
//             .canvas()
//             .draw(Background {
//                 color: Color::RAY_WHITE,
//             })
//             .draw(Text::new(
//                 "move the ball with arrow keys",
//                 Vector2 { x: 10.0, y: 10.0 },
//                 20.0,
//                 Color::DARK_GRAY,
//             ))
//             .draw(CircleGraphic {
//                 circle: &Circle {
//                     center: ball_position,
//                     radius: 50.0,
//                 },
//                 color: Color::MAROON,
//             });
//     }
// }
