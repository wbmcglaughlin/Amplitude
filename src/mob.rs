use bevy::{
    prelude::*,
};

pub const DRAG_CONSTANT: f32 = 0.03;

#[derive(Component)]
pub struct Mob {
    pub(crate) vel: Vec3,
    pub(crate) acc: Vec3,
    pub(crate) health: f32,
    pub(crate) strength: f32
}

impl Mob {
    pub fn update(&mut self, dt: f32) {
        self.vel += self.acc * dt;

        self.vel -= DRAG_CONSTANT * self.vel * self.vel.length();
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