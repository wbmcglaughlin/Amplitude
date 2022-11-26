use bevy::{
    prelude::*,
};
use bevy::app::AppExit;
use bevy::window::close_on_esc;
use iyes_loopless::prelude::*;
use crate::GameState;
use crate::mob::Mob;
use crate::player::Player;
use crate::simulation::Wave;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app   // menu setup (state enter) systems
            .add_enter_system(GameState::MainMenu, setup_menu)
            .add_enter_system(GameState::InGame, game_ui)
            // menu cleanup (state exit) systems
            .add_exit_system(GameState::MainMenu, despawn_with::<MainMenu>)
            .add_exit_system(GameState::InGame, despawn_with::<GameUI>)
            // menu stuff
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::MainMenu)
                    .with_system(close_on_esc)
                    .with_system(butt_interact_visual)
                    // our menu button handlers
                    .with_system(butt_exit.run_if(on_butt_interact::<ExitButt>))
                    .with_system(butt_game.run_if(on_butt_interact::<EnterButt>))
                    .into()
            )// in-game stuff
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::InGame)
                    .with_system(back_to_menu_on_esc)
                    .with_system(update_ui)
                    .into()
            );
    }
}

/// Marker for the main menu entity
#[derive(Component)]
struct GameUI;

// A unit struct to help identify the UI Text Component
#[derive(Component)]
struct UIText;

/// Marker for the main menu entity
#[derive(Component)]
struct MainMenu;

/// Marker for the "Exit App" button
#[derive(Component)]
struct ExitButt;

/// Marker for the "Enter Game" button
#[derive(Component)]
struct EnterButt;

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let butt_style = Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(8.0)),
        margin: UiRect::all(Val::Px(4.0)),
        flex_grow: 1.0,
        ..Default::default()
    };
    let butt_textstyle = TextStyle {
        font: asset_server.load("fonts/framdit.ttf"),
        font_size: 24.0,
        color: Color::BLACK,
    };

    let menu = commands
        .spawn((NodeBundle {
            background_color: BackgroundColor(Color::rgb(0.5, 0.5, 0.5)),
            style: Style {
                size: Size::new(Val::Auto, Val::Auto),
                margin: UiRect::all(Val::Auto),
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        }, MainMenu))
        .id();

    let butt_enter = commands
        .spawn((ButtonBundle {
            style: butt_style.clone(),
            ..Default::default()
        }, EnterButt))
        .with_children(|btn| {
            btn.spawn(TextBundle {
                text: Text::from_section("Enter Game", butt_textstyle.clone()),
                ..Default::default()
            });
        })
        .id();

    let butt_exit = commands
        .spawn((ButtonBundle {
            style: butt_style.clone(),
            ..Default::default()
        }, ExitButt))
        .with_children(|btn| {
            btn.spawn(TextBundle {
                text: Text::from_section("Exit Game", butt_textstyle.clone()),
                ..Default::default()
            });
        })
        .id();

    commands
        .entity(menu)
        .push_children(&[butt_enter, butt_exit]);
}

fn game_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
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
        GameUI
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

/// Transition back to menu on pressing Escape
fn back_to_menu_on_esc(mut commands: Commands, kbd: Res<Input<KeyCode>>) {
    if kbd.just_pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(GameState::MainMenu));
    }
}

/// Despawn all entities with a given component type
pub fn despawn_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

/// Change button color on interaction
fn butt_interact_visual(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *color = BackgroundColor(Color::rgb(0.75, 0.75, 0.75));
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::rgb(0.8, 0.8, 0.8));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::rgb(1.0, 1.0, 1.0));
            }
        }
    }
}

/// Condition to help with handling multiple buttons
///
/// Returns true when a button identified by a given component is clicked.
fn on_butt_interact<B: Component>(
    query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<B>)>,
) -> bool {
    for interaction in query.iter() {
        if *interaction == Interaction::Clicked {
            return true;
        }
    }

    false
}

/// Handler for the Exit Game button
fn butt_exit(mut ev: EventWriter<AppExit>) {
    ev.send(AppExit);
}

/// Handler for the Enter Game button
fn butt_game(mut commands: Commands) {
    // queue state transition
    commands.insert_resource(NextState(GameState::InGame));
}

