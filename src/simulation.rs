use std::collections::HashSet;
use bevy::{
    prelude::*,
};
use bevy::reflect::List;
use rand::{Rng, thread_rng};
use crate::mob::Mob;
use crate::player::Projectile;

pub const MAX_ATTRACTION_DISTANCE: f32 = 100.0;
pub const PLAYER_SIZE: f32 = 1.0;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Wave {
            current: 0
        })
            .add_system(simulation);
    }
}

#[derive(Resource)]
pub struct Wave {
    pub current: usize
}

pub fn simulation(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut wave: ResMut<Wave>,
    mut mobs: Query<(Entity, &mut Transform, &mut Mob), With<Mob>>,
    mut proj: Query<(Entity, &mut Transform, &mut Projectile), (With<Projectile>, Without<Mob>)>,
    time: Res<Time>
) {
    let dt = time.delta_seconds();
    let mut prng = thread_rng();

    let mut despawns = HashSet::new();

    for (entity, mut transform, mut projectile) in proj.iter_mut() {
        let mut proj_accel = Vec3::default();
        for (entity1, transform1, mut mob1) in mobs.iter_mut() {
            let distance = transform1.translation - transform.translation;
            if distance.length_squared() < 0.5 {
                despawns.insert(entity);
                mob1.health -= 10.0;
            }
            proj_accel += (distance)
        }
        projectile.acc = proj_accel.normalize_or_zero();
        projectile.update(dt);

        transform.translation = projectile.pos;
    }

    for entity in despawns {
        commands.entity(entity).despawn();
    }

    if mobs.is_empty() {
        println!("Spawning Wave: {}", wave.current);
        let mobs = get_wave(wave.current);

        for mob_index in 0..mobs {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(prng.gen::<f32>() * 20.0, 0.5,prng.gen::<f32>() * 20.0),
                ..default()
            }).insert(Mob {
                vel: Vec3::default(),
                acc: Vec3::default(),
                health: 10.0,
                strength: 1.0
            });
        }

        wave.current += 1;
    } else {
        let mut forces: Vec<Vec3> = Vec::new();
        let mut damages: Vec<f32> = Vec::new();
        let mut positions: Vec<Vec3> = Vec::new();

        for (entity1, transform1, mob1) in mobs.iter() {
            let mut force = Vec3::default();
            let mut damage = 0.0;

            for (entity2, transform2, mob2) in mobs.iter() {
                if entity2 != entity1 {
                    let distance = transform2.translation - transform1.translation;
                    let distance_squared = distance.length_squared();

                    if distance_squared < MAX_ATTRACTION_DISTANCE * MAX_ATTRACTION_DISTANCE {
                        // Attraction Force
                        force += distance;
                    }

                    // Repel Force
                    let mut repel = 1.0 / distance;
                    repel.y = 0.;

                    force -= repel;
                }
            }

            forces.push(force);
            damages.push(damage);
            positions.push(transform1.translation);
        }

        for (i, (entity, mut transform, mut mob)) in mobs.iter_mut().enumerate() {
            mob.acc = forces[i];
            mob.health -= damages[i] * dt;

            mob.update(dt);

            let mut new_distance = mob.vel * dt;
            let new_position = transform.translation + new_distance;

            for position in &positions {
                if new_position.distance_squared(position.clone()) < PLAYER_SIZE.powf(2.0) / 4.0 {

                }
            }

            transform.translation += new_distance;
            transform.scale.y = mob.strength;

            if mob.health <= 0.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn get_wave(
    wave: usize
) -> usize {
    let mut spawns = 0;

    match wave {
        0 => {spawns = 2},
        1 => {spawns = 3},
        _ => {spawns = wave.pow(2)}
    }

    return spawns;
}

