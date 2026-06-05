use bevy::prelude::{Component, States};

// Game state
#[derive(Default, States, Hash, Debug, Eq, PartialEq, Clone)]
pub enum AppState {
    #[default]
    MainMenu,

    InGame,

    Paused,
}

#[derive(Component)]
pub struct MovementSpeed(pub f32);