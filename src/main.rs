#![allow(non_snake_case)]

// Bevy
use bevy::{
    prelude::*,
    window::PresentMode,
    input::common_conditions::input_just_pressed,
};

// Physics engine
use avian2d::prelude::*;

// Import my custom stuff
pub mod consts;
pub mod components;
mod scoreboard;
mod paddle;
mod ball;

// Main application
fn main() {
    let mut app = App::new();

    // Initialize all game resources
    app.insert_resource(Gravity(Vec2::ZERO));
    app.insert_resource(ClearColor(consts::BG_COLOR));

    // Add all necessary plugins
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bongr".into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }),
        PhysicsPlugins::default(),
        scoreboard::ScoreboardPlugin,
        paddle::PaddlePlugin,
        ball::BallPlugin,
    ));
    
    // Add some general systems
    app.add_systems(Startup, startup);
    app.add_systems(Update, exitSystem.run_if(input_just_pressed(KeyCode::Escape)));

    // Debug build specific stuff
    #[cfg(debug_assertions)]
    {
        use bevy::diagnostic::LogDiagnosticsPlugin;
        app.add_plugins(LogDiagnosticsPlugin::default());
    }

    // Run the application
    app.run();
}

fn startup(mut commands: Commands, window: Single<&Window>) {
    // Spawn a neat 2D camera
    commands.spawn(Camera2d);

    // Top wall
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(window.width(), 1.0),
        Restitution::PERFECTLY_ELASTIC,
        Friction::ZERO,
        Transform::from_xyz(0.0, window.height() / 2.0, 0.0),
    ));

    // Bottom wall
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(window.width(), 1.0),
        Restitution::PERFECTLY_ELASTIC,
        Friction::ZERO,
        Transform::from_xyz(0.0, window.height() / -2.0, 0.0),
    ));

    // Left wall
    commands
        .spawn((
            Sensor,
            CollisionEventsEnabled,
            RigidBody::Static,
            Collider::rectangle(1.0, window.height()),
            Restitution::PERFECTLY_ELASTIC,
            Friction::ZERO,
            Transform::from_xyz(window.width() / -2.0, 0.0, 0.0),
        ))
        .observe(handleHorizontalWallCollision(1));

    // Right wall
    commands
        .spawn((
            Sensor,
            CollisionEventsEnabled,
            RigidBody::Static,
            Collider::rectangle(1.0, window.height()),
            Restitution::PERFECTLY_ELASTIC,
            Friction::ZERO,
            Transform::from_xyz(window.width() / 2.0, 0.0, 0.0),
        ))
        .observe(handleHorizontalWallCollision(0));
}

// Updates the player scores and the scoreboard when the ball hits either the left/right wall
// It's a normal function and not a system so it has to be assigned to the walls manually
// The player scores are a tuple and the function needs the index to figure out which player got the
// score
fn handleHorizontalWallCollision(playerIndex: usize) -> impl Fn(
    On<CollisionStart>, 
    Query<&ball::Ball>, 
    Single<&mut Text, With<scoreboard::Scoreboard>>, 
    ResMut<scoreboard::Scores>
) {
    move |trigger, colliding_with, mut scoreboard, mut scores| {
        if colliding_with.contains(trigger.collider2) {
            // Right wall hit - Player 1 (Index 0) scored
            // Left wall hit - Player 2 (Index 1) scored
            if playerIndex == 0 {
                scores.0 += 1;
            } else {
                scores.1 += 1;
            }

            // Update the scoreboard to display the new scores
            scoreboard.0 = format!("{} : {}", scores.0, scores.1);
        }
    }
}

// Terminate the program with a successful exit code
fn exitSystem(mut msgWriter: MessageWriter<AppExit>) {
    msgWriter.write(AppExit::Success);
}