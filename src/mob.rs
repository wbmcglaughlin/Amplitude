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