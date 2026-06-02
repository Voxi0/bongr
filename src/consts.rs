use bevy::prelude::*;

// Constants
pub const BG_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);

// Paddle
pub const PADDLE_SIZE: Vec2 = Vec2::new(20.0, 200.0);
pub const PADDLE_MOVE_SPEED: f32 = 400.0;
pub const PADDLE_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);

// Ball
pub const BALL_SIZE: f32 = 10.0;
pub const BALL_MOVE_SPEED: f32 = 700.0;
pub const BALL_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);