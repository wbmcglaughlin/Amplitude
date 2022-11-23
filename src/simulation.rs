use std::collections::HashSet;
use bevy::{
    prelude::*,
};
use rand::{Rng, thread_rng};
use crate::mob::{get_mob_type, Mob};
use crate::player::Projectile;

pub const MAX_ATTRACTION_DISTANCE: f32 = 100.0;
pub const PLAYER_SIZE: f32 = 1.0;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Wave {
            current: 0
        })
            .add_system(get_inter_mob_forces)
            .add_system(simulation)
            .add_system(projectile_update);
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
    time: Res<Time>
) {
    // Check if there are any mobs active in the scene, if not begin to spawn next wave.
    if mobs.is_empty() {
        spawn_wave(&mut commands, &mut meshes, &mut materials, wave.current);

        wave.current += 1;

        return;
    }

    let dt: f32 = time.delta_seconds();

    for (entity, mut transform, mut mob) in mobs.iter_mut(){
        mob.update(dt);

        transform.translation = mob.pos;
        transform.scale.y = mob.strength;

        if mob.health <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn get_inter_mob_forces(
    mut mobs: Query<(Entity, &mut Transform, &mut Mob), With<Mob>>,
) {
    let mut forces = Vec::new();

    // Time step for current frame
    for (entity1, transform1, mob1) in mobs.iter() {
        let mut force = Vec3::default();

        for (entity2, transform2, mob2) in mobs.iter() {
            if entity2 != entity1 {
                let distance = transform2.translation - transform1.translation;
                let distance_squared = distance.length_squared();

                if distance_squared < MAX_ATTRACTION_DISTANCE * MAX_ATTRACTION_DISTANCE {
                    // Attraction Force
                    force += distance;
                }

                // Repel Force
                let mut repel = 3.0 / distance;
                repel.y = 0.;

                force -= repel;
            }
        }

        forces.push(force);
    }

    for (i, (_, _, mut mob)) in mobs.iter_mut().enumerate() {
        mob.force = forces[i];
    }
}

fn projectile_update(
    mut commands: Commands,
    time: Res<Time>,
    mut mobs: Query<(Entity, &mut Transform, &mut Mob), With<Mob>>,
    mut proj: Query<(Entity, &mut Transform, &mut Projectile), (With<Projectile>, Without<Mob>)>,
) {
    let dt = time.delta_seconds();

    // Handle the projectile interaction with the mobs. If projectiles need to be de-spawned after the force analysis
    // they can be added to the hashset.
    let mut despawns = HashSet::new();

    for (entity, mut transform, mut projectile) in proj.iter_mut() {
        let mut proj_accel = Vec3::default();
        for (_, transform1, mut mob1) in mobs.iter_mut() {
            let distance = transform1.translation - transform.translation;
            if distance.length_squared() < 0.5 {
                despawns.insert(entity);
                mob1.health -= projectile.damage;
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
}

fn spawn_wave (
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    wave_number: usize
) {
    // Random number generation for the simulation
    let mut prng = thread_rng();

    let mobs = get_wave(wave_number);

    for _ in 0..mobs {
        let strength = 1.0;
        let position = Vec3::new(prng.gen::<f32>() * 20.0,
                                 0.5,
                                 prng.gen::<f32>() * 20.0);

        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(get_mob_type(strength).into()),
            transform: Transform::from_translation(position),
            ..default()
        }).insert(Mob {
            pos: position,
            vel: Vec3::default(),
            acc: Vec3::default(),
            force: Vec3::default(),
            health: 10.0,
            strength,
            mass: 1.0
        });
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

