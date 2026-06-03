use bevy::prelude::*;
use avian2d::prelude::*;
use crate::consts::*;

// Random number generator
use rand::prelude::*;

// Ball component
#[derive(Component)]
pub struct Ball;

// Ball plugin
pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initBall);
    }
}

fn initBall(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Randomly generate the X and Y direction for the ball
    let ballLaunchDir: Vec2 = loop {
        let (randomXDir, randomYDir): (f32, f32) = (
            rand::rng().random_range(-1..=1) as f32,
            rand::rng().random_range(-1..=1) as f32,
        );
        
        // Return the direction only if both X and Y properties are non-zero values
        if randomXDir != 0.0 && randomYDir != 0.0 {
            break Vec2::new(randomXDir, randomYDir);
        }
    };

    // Spawn the ball
    commands.spawn((
        Ball,
        RigidBody::Dynamic,
        Collider::circle(BALL_SIZE),
        Friction::ZERO,
        Restitution::new(1.0),
        LinearVelocity(ballLaunchDir * BALL_MOVE_SPEED),

        // Prevents the ball from rotating when clipped by a paddle corner
        LockedAxes::ROTATION_LOCKED,

        Mesh2d(meshes.add(Circle::new(BALL_SIZE))),
        MeshMaterial2d(materials.add(BALL_COLOR)),
    ));
}