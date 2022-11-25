use bevy::{
    prelude::*,
};
use crate::mob::Mob;
use crate::player::Player;
use crate::simulation::Wave;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ui)
            .add_system(update_ui);
    }
}

// A unit struct to help identify the UI Text Component
#[derive(Component)]
struct UIText;

fn spawn_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    // UI camera
    commands.spawn(Camera2dBundle {
        camera: Camera {
            priority: 0,
            ..default()
        },
        ..default()
    });
    // Text with one section
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_sections([
            TextSection::new(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "100",
            TextStyle {
                font: asset_server.load("fonts/framdit.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
                },
            ),
            TextSection::new(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "\n0",
                TextStyle {
                    font: asset_server.load("fonts/framdit.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            )
        ]),
        UIText,
    ));
}

fn update_ui(
    mut players: Query<(&mut Player), (Without<Mob>, With<Player>)>,
    mut text: Query<(&mut Text), With<UIText>>,
    wave: Res<Wave>
) {
    for (mut text) in &mut text {
        for (mut player) in players.iter_mut() {
            text.sections[0].value = format!(
                "{:.1} health",
                player.health
            );

            text.sections[1].value = format!(
                "\nwave {:.1}",
                wave.current
            );
        }
    }
}