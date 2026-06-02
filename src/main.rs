#![allow(non_snake_case)]

// Bevy
use bevy::{
    prelude::*,
    input::common_conditions::input_just_pressed,
};

// Physics engine
use avian2d::prelude::*;

// Import my custom stuff
pub mod consts;
pub mod components;
mod paddle;
mod ball;

// Main application
fn main() {
    App::new()
        .insert_resource(Gravity(Vec2::ZERO))
        .insert_resource(ClearColor(consts::BG_COLOR))
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            paddle::PaddlePlugin,
            ball::BallPlugin,
        ))
        .add_systems(Startup, startup)
        .add_systems(Update, exitSystem.run_if(input_just_pressed(KeyCode::Escape)))
        .run();
}

fn startup(mut commands: Commands, window: Single<&Window>) {
    // Spawn a neat 2D camera
    commands.spawn(Camera2d);

    let wall_thickness = 10.0;

    // --- TOP WALL ---
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(window.width(), wall_thickness),
        Restitution::PERFECTLY_ELASTIC,
        Friction::ZERO,
        Transform::from_xyz(0.0, window.height() / 2.0, 0.0),
    ));

    // --- BOTTOM WALL ---
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(window.width(), wall_thickness),
        Restitution::PERFECTLY_ELASTIC,
        Friction::ZERO,
        Transform::from_xyz(0.0, window.height() / -2.0, 0.0),
    ));

    // --- LEFT WALL ---
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(wall_thickness, window.height()),
        Restitution::PERFECTLY_ELASTIC,
        Friction::ZERO,
        Transform::from_xyz(window.width() / -2.0, 0.0, 0.0),
    ));

    // --- RIGHT WALL ---
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(wall_thickness, window.height()),
        Restitution::PERFECTLY_ELASTIC,
        Friction::ZERO,
        Transform::from_xyz(window.width() / 2.0, 0.0, 0.0),
    ));
}

// Terminate the program if the escape key is pressed
fn exitSystem(mut msgWriter: MessageWriter<AppExit>) {
    msgWriter.write(AppExit::Success);
}