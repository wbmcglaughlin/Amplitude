use std::time::Duration;
use bevy::{
    prelude::*,
};
use bevy::time::Stopwatch;
use bevy_mod_raycast::RaycastSource;
use iyes_loopless::prelude::*;
use crate::GameState;
use crate::surface::{CAMERA_DISTANCE, GameCamera, Surface};
use crate::ui::despawn_with;

pub const GRAVITY: f32 = -1.;
pub const SPEED: f32 = 0.3;
pub const SIDE_SPEED_FACTOR: f32 = 1.;
pub const JUMP_ACCEL: f32 = -GRAVITY * 40.;
pub const JUMP_TIMER: f32 = 0.2;

pub const PLAYER_COLOUR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const TARGET_COLOUR: Color = Color::rgba(0.9, 0.9, 0.9, 0.3);

pub const PROJECTILE_SPAWN_RATE: f32 = 0.3;
pub const PROJECTILE_SPEED: f32 = 5.0;
pub const PROJECTILE_LIFETIME: f32 = 10.0;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::InGame, spawn_player)
            .add_exit_system(GameState::MainMenu, despawn_with::<Player>)
            .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(player_control)
                .with_system(handle_mouse_clicks)
                .with_system(projectile_spawner)
                .into()
            );
    }
}

#[derive(Component, Default)]
pub struct Player {
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
    pub health: f32,
    pub target_position: Vec3,
    pub last_jump: Stopwatch
}

impl Player {
    pub fn update(
        &mut self,
        dt: f32
    ) {
        // Get current direction and slow down
        let mut cd = 0.04;

        let difference = (self.target_position - self.pos);
        let length = difference.length_squared();

        self.acc = difference;

        self.vel += self.acc * dt;

        cd = (1.0 - cd) / (1.0 + length) + cd;

        self.vel -= cd * self.vel * self.vel.length() * dt;

        self.pos += self.vel * dt;
        self.pos.y = 0.5;
    }
}

#[derive(Component)]
pub struct Projectile {
    pub pos: Vec3,
    pub vel: Vec3,
    pub acc: Vec3,
    pub damage: f32,
    pub lifetime: Timer
}

impl Projectile {
    pub fn update(
        &mut self,
        dt: f32
    ) {
        // Get current direction and slow down
        let cd = 0.04;

        self.vel += self.acc * dt * PROJECTILE_SPEED;
        self.vel -= cd * self.vel * self.vel.length() * dt;

        self.pos += self.vel * dt;
        self.pos.y = 0.5;
    }
}

#[derive(Component)]
pub struct ProjectileTimer {
    timer: Timer,
}

#[derive(Component)]
pub struct Target;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(PLAYER_COLOUR.into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    }).insert(Player {
        pos: Vec3::new(0.0, 0.5, 0.0),
        vel: Vec3::default(),
        acc: Vec3::default(),
        health: 100.0,
        target_position: Vec3::new(0.0, 0.5, 0.0),
        ..default()
    }).insert(ProjectileTimer {
        timer: Timer::new(Duration::from_secs_f32(PROJECTILE_SPAWN_RATE), TimerMode::Repeating)
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.3 })),
        material: materials.add(TARGET_COLOUR.into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    }).insert(Target {
    });
}

pub fn player_control(
    time: Res<Time>,
    mut camera: Query<(&mut Transform), (With<GameCamera>, Without<Player>)>,
    mut player_query: Query<(Entity, &mut Transform, &mut Player, &mut ProjectileTimer), With<Player>>
) {
    for (entity, mut transform, mut player, mut timer) in player_query.iter_mut() {
        player.update(time.delta_seconds());
        transform.translation = player.pos;

        for (mut camera_transform) in camera.iter_mut() {
            *camera_transform = Transform::from_translation(
                    Vec3::new(CAMERA_DISTANCE / 1.5,
                             1.0 * CAMERA_DISTANCE,
                             CAMERA_DISTANCE / 1.5) + transform.translation
                )
                .looking_at(transform.translation, Vec3::Y)
        }
    }
}

fn handle_mouse_clicks(
    mut player_query: Query<(Entity, &mut Transform, &mut Player), With<Player>>,
    mut target: Query<&mut Transform, (With<Target>, Without<Player>)>,
    mouse_input: Res<Input<MouseButton>>,
    to: Query<&RaycastSource<Surface>>,
) {
    if let Ok(raycast_source) = to.get_single() {
        if let Some(top_intersection) = raycast_source.get_nearest_intersection() {
            let mut new_position = top_intersection.1.position();
            new_position.y = 0.5;
            if mouse_input.just_pressed(MouseButton::Left) {
                for (entity, mut transform, mut player) in player_query.iter_mut() {
                    player.target_position = new_position;
                    for mut transform in target.iter_mut() {
                        transform.translation = new_position;
                    }
                }
            }
        }
    }
}

fn projectile_spawner(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    mut player_query: Query<(Entity, &mut Transform, &mut Player, &mut ProjectileTimer), With<Player>>
) {
    for (entity, mut transform, mut player, mut timer) in player_query.iter_mut() {
        timer.timer.tick(time.delta());
        if timer.timer.finished() {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.3 })),
                material: materials.add(PLAYER_COLOUR.into()),
                transform: Transform::from_translation(transform.translation),
                ..default()
            }).insert(Projectile {
                pos: transform.translation,
                vel: Vec3::default(),
                acc: Vec3::default(),
                damage: 3.0,
                lifetime: Timer::new(Duration::from_secs_f32(PROJECTILE_LIFETIME), TimerMode::Once),
            });
        }
    }
}