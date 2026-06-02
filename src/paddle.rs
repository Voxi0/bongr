use bevy::prelude::*;

use crate::components::*;

// Constants
const PADDLE_SIZE: Vec2 = Vec2::new(20.0, 200.0);

#[derive(Component)]
struct Paddle {
    upKey: KeyCode,
    downKey: KeyCode,
}

// Paddle plugin
pub struct PaddlePlugin;
impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initPaddles);
        app.add_systems(Update, updatePaddle);
    }
}

// Creates the 2 paddles/players
fn initPaddles(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, window: Single<&Window>) {
    // Paddle properties
    let paddleSize: Vec2 = Vec2::new(20.0, 200.0);
    let paddleMesh = meshes.add(Rectangle::new(paddleSize.x, paddleSize.y));
    let paddleMaterial = materials.add(Color::srgba(1.0, 0.0, 0.0, 1.0));

    // In Bevy, (0,0) refers to the center of the screen
    // We do these calculations to properly figure out where to position the paddles
    // I'm just used to (0,0) being top left since that's traditional
    let windowLeftEdge: f32 = window.width() / -2.0;
    let windowRightEdge: f32 = window.width() / 2.0;

    // Create the 2 paddles/players
    commands.spawn((
        Paddle {
            upKey: KeyCode::KeyW,
            downKey: KeyCode::KeyS,
        },
        MovementSpeed(300.0),
        Mesh2d(paddleMesh.clone()),
        MeshMaterial2d(paddleMaterial.clone()),
        Transform {
            translation: Vec3::new(windowLeftEdge + (paddleSize.x + 10.0), 0.0, 0.0),
            ..default()
        }
    ));
    commands.spawn((
        Paddle {
            upKey: KeyCode::KeyW,
            downKey: KeyCode::KeyS,
        },
        MovementSpeed(300.0),
        Mesh2d(paddleMesh.clone()),
        MeshMaterial2d(paddleMaterial.clone()),
        Transform {
            translation: Vec3::new(windowRightEdge - (paddleSize.x + 10.0), 0.0, 0.0),
            ..default()
        }
    ));
}

fn updatePaddle(mut query: Query<(&mut Transform, &Paddle, &MovementSpeed)>, keys: Res<ButtonInput<KeyCode>>, time: Res<Time>, window: Single<&Window>) {
    for (mut transform, paddle, moveSpeed) in query.iter_mut() {
        // Movement
        if keys.pressed(paddle.upKey) {
            transform.translation.y += moveSpeed.0 * time.delta_secs();
        }
        if keys.pressed(paddle.downKey) {
            transform.translation.y -= moveSpeed.0 * time.delta_secs();
        }

        // Stop the paddle from moving out of the screen
        transform.translation.y = transform.translation.y.clamp((window.height() - PADDLE_SIZE.y) / -2.0, (window.height() - PADDLE_SIZE.y) / 2.0);
    }
}