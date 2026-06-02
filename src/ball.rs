use bevy::prelude::*;
use avian2d::prelude::*;
use crate::consts::*;
use crate::components::*;

// Ball component
#[derive(Component)]
struct Ball {
    moveDir: Vec2,
}

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
    // Spawn the ball
    let ballLaunchDir: Vec2 = Vec2::new(1.0, 1.0);
    commands.spawn((
        Ball {moveDir: Vec2::new(1.0, 1.0)},
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