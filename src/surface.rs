use std::f32::consts::PI;
use bevy::{
    prelude::*,
};
use bevy::render::camera::ScalingMode;
use bevy_mod_raycast::{DefaultPluginState, DefaultRaycastingPlugin, RaycastMesh, RaycastMethod, RaycastSource, RaycastSystem};

pub const GROUND_COLOR: f32 = 0.1;
pub const GROUND_SIZE: f32 = 16.0;
pub const CAMERA_DISTANCE: f32 = GROUND_SIZE * 0.8;

pub const GROUND_PLANES: i32 = 3;

pub struct SurfacePlugin;

impl Plugin for SurfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_lighting)
            .add_startup_system(generate_world)
            .add_system(update_raycast_with_cursor)
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
            );
    }
}

#[derive(Component)]
pub struct GameCamera;

/// This is a unit struct we will use to mark our generic `RaycastMesh`s and `RaycastSource` as part
/// of the same group, or "RaycastSet". For more complex use cases, you might use this to associate
/// some meshes with one ray casting source, and other meshes with a different ray casting source."
pub struct Surface;

// Update our `RaycastSource` with the current cursor position every frame.
fn update_raycast_with_cursor(
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

pub fn generate_lighting(
    mut commands: Commands,
) {
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

pub fn generate_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(DefaultPluginState::<Surface>::default());

    for plane_x in (-GROUND_PLANES+1)..GROUND_PLANES {
        for plane_y in (-GROUND_PLANES+1)..GROUND_PLANES {
            // Ground
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: GROUND_SIZE })),
                material: materials.add(
                    Color::rgb(
                        GROUND_COLOR + (plane_x.abs() as f32 * 0.1),
                        GROUND_COLOR,
                        GROUND_COLOR
                    ).into()),
                transform: Transform {
                    translation: Vec3::new(
                        plane_x as f32 * GROUND_SIZE ,
                        0f32,
                        plane_y as f32 * GROUND_SIZE),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    ..default()
                },
                ..default()
            }).insert(RaycastMesh::<Surface>::default()); // Make this mesh ray cast-able
        }
    }

    commands.spawn(Camera3dBundle {
        camera: Camera {
          priority: 1,
            ..default()
        },
        projection: OrthographicProjection {
            scale: 3.0,
            scaling_mode: ScalingMode::FixedVertical(CAMERA_DISTANCE),
            ..default()
        }.into(),
        transform: Transform::from_xyz(
            CAMERA_DISTANCE / 1.5,
            1.0 * CAMERA_DISTANCE,
            CAMERA_DISTANCE / 1.5
        )
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
        })
            .insert(GameCamera {})
            .insert(RaycastSource::<Surface>::new()); // Designate the camera as our source
}