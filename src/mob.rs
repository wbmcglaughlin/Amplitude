use bevy::{
    prelude::*,
};

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
    }
}