#![allow(non_snake_case)]

// Bevy
use bevy::{
    prelude::*,
    input::common_conditions::input_just_pressed,
};

pub mod consts;
pub mod components;
mod paddle;

// Main application
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(paddle::PaddlePlugin)
        .add_systems(Startup, startup)
        .add_systems(Update, exitSystem.run_if(input_just_pressed(KeyCode::Escape)))
        .run();
}

fn startup(mut commands: Commands) {
    // Spawn a neat 2D camera
    commands.spawn(Camera2d);
}

// Terminate the program if the escape key is pressed
fn exitSystem(mut msgWriter: MessageWriter<AppExit>) {
    msgWriter.write(AppExit::Success);
}