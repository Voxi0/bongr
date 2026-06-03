use bevy::prelude::*;

// Player scores
// First element is player 1's scores
// Second element is player 2's scores
#[derive(Resource)]
pub struct Scores(pub usize, pub usize);

// Just a tag component to easily identify the scoreboard
#[derive(Component)]
pub struct Scoreboard;

// Scoreboard plugin
pub struct ScoreboardPlugin;
impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scores(0, 0));
        app.add_systems(Startup, createScoreboard);
    }
}

// Create the scoreboard
fn createScoreboard(mut commands: Commands, scores: Res<Scores>) {
    // The scoreboard
    commands.spawn(Node {
        position_type: PositionType::Absolute,
        justify_content: JustifyContent::Center,
        top: Val::Px(20.0),
        left: Val::Px(0.0),
        right: Val::Px(0.0),
        ..default()
    }).with_children(|parent| {
        parent.spawn((
            Scoreboard,
            Text::new(format!("{} : {}", scores.0, scores.1)),
            TextColor(Color::WHITE),
        ));
    });
}