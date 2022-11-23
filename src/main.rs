mod surface;
mod mob;
mod simulation;
mod player;

use bevy::{
    prelude::*,
};
use bevy_mod_raycast::{DefaultPluginState, DefaultRaycastingPlugin, RaycastMethod, RaycastSource, RaycastSystem};

use crate::player::PlayerPlugin;
use crate::simulation::SimulationPlugin;
use crate::surface::{generate_world, Surface};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(generate_world)
        .add_plugin(SimulationPlugin)
        .add_plugin(PlayerPlugin)
        // The DefaultRaycastingPlugin bundles all the functionality you might need into a single
        // plugin. This includes building rays, casting them, and placing a debug cursor at the
        // intersection. For more advanced uses, you can compose the systems in this plugin however
        // you need. For example, you might exclude the debug cursor system.
        .add_plugin(DefaultRaycastingPlugin::<Surface>::default())
        // Ray casting should probably happen near
        // start of the frame. For example, we want to be sure this system runs before we construct
        // any rays, hence the ".before(...)". You can use these provided RaycastSystem labels to
        // order your systems with the ones provided by the raycasting plugin.
        .add_system_to_stage(
            CoreStage::First,
            update_raycast_with_cursor.before(RaycastSystem::BuildRays::<Surface>),
        )
        .run();
}

// Update our `RaycastSource` with the current cursor position every frame.
pub fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RaycastSource<Surface>>,
) {
    // Grab the most recent cursor event if it exists:
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };

    for mut pick_source in &mut query {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_position);
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
) {
    // .with_debug_cursor() - add this if needed.
    commands.insert_resource(DefaultPluginState::<Surface>::default());

    // Insert lighting to frame
    const HALF_SIZE: f32 = 40.0;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 30000.0,
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 2.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_6) +
                Quat::from_rotation_y(-std::f32::consts::FRAC_PI_6),
            ..default()
        },
        ..default()
    });

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });
}