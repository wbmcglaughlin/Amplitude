use bevy::{
    prelude::*,
};
use bevy::time::Stopwatch;
use bevy_mod_raycast::RaycastSource;
use crate::surface::{GameCamera, Surface};

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
            .add_system(handle_mouse_clicks);
    }
}

#[derive(Component, Default)]
pub struct Player {
    vel: Vec3,
    acc: Vec3,
    health: f32,

    pub last_jump: Stopwatch
}

impl Player {
    pub fn update(
        &mut self,
        forward: f32,
        up: f32,
        side: f32,
        dt: f32
    ) {
        // Get current direction and slow down
        let cd = 0.04;

        self.acc = Vec3::new(
            forward,
            up + GRAVITY,
            side);

        self.vel += self.acc * dt;

        self.vel -= cd * self.vel * self.vel.length();
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.2, 0.3, 0.2).into()),
        transform: Transform::from_xyz(0.0, 0.5,0.0),
        ..default()
    }).insert(Player {
        vel: Vec3::default(),
        acc: Vec3::default(),
        health: 100.0,
        ..default()
    });
}

pub fn player_control(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(Entity, &mut Transform, &mut Player), With<Player>>
) {
    for (entity, mut transform, mut player) in player_query.iter_mut() {
        let mut forward = 0f32;
        let mut side = 0f32;
        let mut up = 0f32;

        if keyboard_input.pressed(KeyCode::W) {
            forward += SPEED;
        }
        if keyboard_input.pressed(KeyCode::A) {
            side += SPEED * SIDE_SPEED_FACTOR;
        }
        if keyboard_input.pressed(KeyCode::S) {
            forward -= SPEED * SIDE_SPEED_FACTOR;
        }
        if keyboard_input.pressed(KeyCode::D) {
            side -= SPEED * SIDE_SPEED_FACTOR;
        }

        // Jump handler
        if keyboard_input.pressed(KeyCode::Space) {
            if player.last_jump.elapsed_secs() > JUMP_TIMER {
                player.last_jump.reset();
                up += JUMP_ACCEL;
            }
        }

        player.last_jump.tick(time.delta());

        player.update(forward, up, side, time.delta_seconds());
        transform.translation += player.vel;

        if transform.translation.y < 0.5 {
            transform.translation.y = 0.5;
        }
    }
}

fn handle_mouse_clicks(
    camera: Query<(&Projection, &Transform, &GameCamera), With<GameCamera>>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    to: Query<&RaycastSource<Surface>>,
) {
    let win = windows.get_primary().expect("no primary window");

    if mouse_input.just_pressed(MouseButton::Left) {
        let cursor_position = win.cursor_position();

        for (projection, transform, game_camera) in camera.iter() {

        }
    }
}