use std::f32::consts::PI;
use bevy::{
    prelude::*,
};
use bevy::render::camera::ScalingMode;
use bevy_mod_raycast::{
    RaycastMesh, RaycastMethod, RaycastSource,
};

pub const GROUND_COLOR: f32 = 0.1;
pub const GROUND_SIZE: f32 = 16.0;
pub const CAMERA_DISTANCE: f32 = GROUND_SIZE * 0.8;

pub const GROUND_PLANES: i32 = 3;

#[derive(Component)]
pub struct GameCamera {}

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

pub fn generate_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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