use std::f32::consts::PI;
use bevy::{
    prelude::*,
};

pub const GROUND_COLOR: f32 = 0.1;
pub const GROUND_SIZE: f32 = 32.0;
pub const CAMERA_DISTANCE: f32 = GROUND_SIZE * 0.8;

pub const GROUND_PLANES: i32 = 3;

pub fn generate_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for plane_x in (-GROUND_PLANES+1)..GROUND_PLANES {
        for plane_y in (-GROUND_PLANES+1)..GROUND_PLANES {
            // Ground
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: GROUND_SIZE })),
                material: materials.add(Color::rgb(GROUND_COLOR, GROUND_COLOR, GROUND_COLOR).into()),
                transform: Transform {
                    translation: Vec3::new(plane_x as f32 * GROUND_SIZE , 0f32, plane_y as f32 * GROUND_SIZE),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    ..default()
                },
                ..default()
            });
        }
    }

    commands.spawn(Camera3dBundle {
        projection: bevy::render::camera::Projection::Perspective(PerspectiveProjection {
            fov: PI / 3.,
            far: 2048.0,
            ..Default::default()
        }),
        transform: Transform::from_xyz(CAMERA_DISTANCE, CAMERA_DISTANCE, CAMERA_DISTANCE).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}