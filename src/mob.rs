use std::time::Duration;
use bevy::{
    prelude::*,
};
use bevy::time::Stopwatch;

pub const DRAG_CONSTANT: f32 = 0.03;

pub const ATTACKED_COLOR: Color = Color::rgb(0.9, 0.4, 0.4);
pub const ATTACKED_FLASH_TIME: f32 = 0.5;

#[derive(Component)]
pub struct Mob {
    pub(crate) pos: Vec3,
    pub(crate) vel: Vec3,
    pub(crate) acc: Vec3,
    pub(crate) force: Vec3,
    pub(crate) health: f32,
    pub(crate) strength: f32,
    pub(crate) mass: f32,

    pub color: Color,
    pub last_damaged: Stopwatch
}

impl Mob {
    pub fn update(&mut self, dt: f32) {
        // Apply force to mob
        self.acc = self.mass * self.force;
        self.vel += self.acc * dt;
        self.vel -= DRAG_CONSTANT * self.vel * self.vel.length();
        self.pos += self.vel * dt;

        // Reset force
        self.force = Vec3::default();

        // Tick last damaged timer
        self.last_damaged.tick(Duration::from_secs_f32(dt));
    }

    pub fn damage(
        &mut self,
        damage: f32,
    ) {
        // Damage the mob
        self.health -= damage;

        // Reset last damaged timer
        self.last_damaged.reset();
    }
}

pub fn get_mob_type(
    strength: f32
) -> Color {
    let strength_type = strength as i32;

    if strength_type < 1 {
        return Color::rgb(1.0, 1.0, 1.0);
    } else if strength_type < 2 {
        return Color::rgb(1.0, 0.9, 0.9);
    }

    return Color::rgb(1.0, 0.6, 0.6);
}