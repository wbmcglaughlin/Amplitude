mod surface;
mod mob;
mod simulation;
mod player;
mod ui;

use bevy::{
    prelude::*,
};
use iyes_loopless::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::window::PresentMode;

use crate::player::PlayerPlugin;
use crate::simulation::SimulationPlugin;
use crate::surface::SurfacePlugin;
use crate::ui::UIPlugin;

/// Our Application State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    MainMenu,
    InGame,
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Amplitude".to_string(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_loopless_state(GameState::MainMenu)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(SurfacePlugin)
        .add_plugin(SimulationPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(UIPlugin)
        .run();
}