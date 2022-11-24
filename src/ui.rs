use bevy::{
    prelude::*,
};
use crate::mob::Mob;
use crate::player::Player;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ui)
            .add_system(update_ui);
    }
}

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
struct ColorText;

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
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "100",
            TextStyle {
                font: asset_server.load("fonts/framdit.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::TOP_CENTER)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        ColorText,
    ));
}

fn update_ui(
    mut players: Query<(&mut Player), (Without<Mob>, With<Player>)>,
    mut text: Query<(&mut Text), With<ColorText>>
) {
    for (mut text) in &mut text {
        for (mut player) in players.iter_mut() {
            text.sections[0].value = format!(
                "{:.1} health",
                player.health
            );
        }
    }
}