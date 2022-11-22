use std::time::Duration;
use bevy::{
    prelude::*,
};
use bevy::time::Stopwatch;
use bevy_mod_raycast::RaycastSource;
use crate::surface::Surface;

pub const GRAVITY: f32 = -1.;
pub const SPEED: f32 = 0.3;
pub const SIDE_SPEED_FACTOR: f32 = 1.;
pub const JUMP_ACCEL: f32 = -GRAVITY * 40.;
pub const JUMP_TIMER: f32 = 0.2;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_control)
            .add_system(handle_mouse_clicks)
            .add_system(projectile_spawner);
    }
}

#[derive(Component, Default)]
pub struct Player {
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
    health: f32,
    pub target_position: Vec3,
    pub last_jump: Stopwatch
}

impl Player {
    pub fn update(
        &mut self,
        dt: f32
    ) {
        // Get current direction and slow down
        let cd = 0.04;

        self.acc = (self.target_position - self.pos).normalize_or_zero();

        self.vel += self.acc * dt;
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
}

impl Projectile {
    pub fn update(
        &mut self,
        dt: f32
    ) {
        // Get current direction and slow down
        let cd = 0.04;

        self.vel += self.acc * dt;
        self.vel -= cd * self.vel * self.vel.length() * dt;

        self.pos += self.vel * dt;
        self.pos.y = 0.5;
    }
}

#[derive(Component)]
pub struct ProjectileTimer {
    timer: Timer,
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.2, 0.3, 0.2).into()),
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
        timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating)
    });
}

pub fn player_control(
    time: Res<Time>,
    mut player_query: Query<(Entity, &mut Transform, &mut Player, &mut ProjectileTimer), With<Player>>
) {
    for (entity, mut transform, mut player, mut timer) in player_query.iter_mut() {
        player.update(time.delta_seconds());
        transform.translation = player.pos;
    }
}

fn handle_mouse_clicks(
    mut player_query: Query<(Entity, &mut Transform, &mut Player), With<Player>>,
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
                material: materials.add(Color::rgb(0.2, 0.3, 0.2).into()),
                transform: Transform::from_xyz(0.0, 0.15, 0.0),
                ..default()
            }).insert(Projectile {
                pos: transform.translation,
                vel: Vec3::default(),
                acc: Vec3::default(),
            });
        }
    }
}