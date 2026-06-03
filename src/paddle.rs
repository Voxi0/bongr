use bevy::prelude::*;
use avian2d::prelude::*;
use crate::components::*;
use crate::consts::*;

// Paddle component
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
        app.add_systems(FixedUpdate, updatePaddle);
    }
}

// Creates the 2 paddles/players
fn initPaddles(
    mut commands: Commands,
    window: Single<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Paddle properties
    let paddleMesh = meshes.add(Rectangle::new(PADDLE_SIZE.x, PADDLE_SIZE.y));
    let paddleMaterial = materials.add(PADDLE_COLOR);

    // In Bevy, (0,0) refers to the center of the screen
    // We do these calculations to properly figure out where to position the paddles
    // I'm just used to (0,0) being top left since that's traditional
    let windowLeftEdge: f32 = window.width() / -2.0;
    let windowRightEdge: f32 = window.width() / 2.0;

    // Create the 2 paddles/players
    commands.spawn((
        Mesh2d(paddleMesh.clone()),
        MeshMaterial2d(paddleMaterial.clone()),
        RigidBody::Kinematic,
        Collider::rectangle(PADDLE_SIZE.x, PADDLE_SIZE.y),
        Friction::ZERO,
        Restitution::PERFECTLY_ELASTIC,
        MovementSpeed(PADDLE_MOVE_SPEED),
        Paddle {
            upKey: KeyCode::KeyW,
            downKey: KeyCode::KeyS,
        },
        Transform {
            translation: Vec3::new(windowLeftEdge + (PADDLE_SIZE.x + 10.0), 0.0, 0.0),
            ..default()
        }
    ));
    commands.spawn((
        Mesh2d(paddleMesh.clone()),
        MeshMaterial2d(paddleMaterial.clone()),
        RigidBody::Kinematic,
        Collider::rectangle(PADDLE_SIZE.x, PADDLE_SIZE.y),
        Friction::ZERO,
        Restitution::PERFECTLY_ELASTIC,
        MovementSpeed(PADDLE_MOVE_SPEED),
        Paddle {
            upKey: KeyCode::ArrowUp,
            downKey: KeyCode::ArrowDown,
        },
        Transform {
            translation: Vec3::new(windowRightEdge - (PADDLE_SIZE.x + 10.0), 0.0, 0.0),
            ..default()
        }
    ));
}

// Handle paddle/player movement and such
fn updatePaddle(
    mut query: Query<(&mut Position, &Paddle, &MovementSpeed)>,
    window: Single<&Window>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut position, paddle, moveSpeed) in query.iter_mut() {
        // Movement
        if keys.pressed(paddle.upKey) {
            position.y += moveSpeed.0 * time.delta_secs();
        }
        if keys.pressed(paddle.downKey) {
            position.y -= moveSpeed.0 * time.delta_secs();
        }

        // Stop the paddle from moving out of the screen
        position.y = position.y.clamp((window.height() - PADDLE_SIZE.y) / -2.0, (window.height() - PADDLE_SIZE.y) / 2.0);
    }
}