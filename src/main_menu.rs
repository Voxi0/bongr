use bevy::prelude::*;
use crate::components::AppState;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), mainMenuSetup);
        app.add_systems(Update, mainMenuUpdate.run_if(in_state(AppState::MainMenu)));
    }
}

fn mainMenuSetup(mut commands: Commands) {}