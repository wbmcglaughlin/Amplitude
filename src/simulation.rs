use std::collections::HashSet;

use bevy::{
    prelude::*,
};
use bevy::time::Stopwatch;
use rand::{Rng, thread_rng};
use crate::mob::{ATTACKED_COLOR, ATTACKED_FLASH_TIME, get_mob_type, Mob};
use crate::player::{Player, Projectile};

pub const MAX_ATTRACTION_DISTANCE: f32 = 10.0;
pub const PLAYER_SIZE: f32 = 1.0;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Wave {
            current: 0
        })
            .add_system(get_inter_mob_forces)
            .add_system(get_player_mob_forces)
            .add_system(player_mob_interaction)
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
    mut mobs: Query<(Entity, &mut Transform, &mut Mob, &Handle<StandardMaterial>), With<Mob>>,
    time: Res<Time>
) {
    // Check if there are any mobs active in the scene, if not begin to spawn next wave.
    if mobs.is_empty() {
        spawn_wave(&mut commands, &mut meshes, &mut materials, wave.current);

        wave.current += 1;

        return;
    }

    let dt: f32 = time.delta_seconds();

    for (entity, mut transform, mut mob, material_handle) in mobs.iter_mut(){
        mob.update(dt);

        transform.translation = mob.pos;
        transform.scale.y = mob.strength;

        if mob.health <= 0.0 {
            commands.entity(entity).despawn();
        }

        if mob.last_damaged.elapsed_secs() < ATTACKED_FLASH_TIME {
            let material = materials.get_mut(material_handle).unwrap();

            material.base_color = color_lerp(ATTACKED_COLOR, mob.color, (mob.last_damaged.elapsed_secs()/ATTACKED_FLASH_TIME).min(1.));
        }
    }
}

fn player_mob_interaction(
    mut mobs: Query<(Entity, &mut Transform, &mut Mob), With<Mob>>,
    mut players: Query<(Entity, &mut Transform, &mut Player), (Without<Mob>, With<Player>)>,
    time: Res<Time>
) {
    for (_, transform, mut mob) in mobs.iter_mut() {
        for (_, p_transform, mut player) in players.iter_mut() {
            let distance = (p_transform.translation - transform.translation).length_squared();

            if distance < 2.0_f32.powf(2.0) {
                player.health -= mob.strength * time.delta_seconds();
            }
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
        mob.force += forces[i];
    }
}

fn get_player_mob_forces(
    mut mobs: Query<(Entity, &mut Transform, &mut Mob), With<Mob>>,
    mut players: Query<(Entity, &mut Transform, &mut Player), (Without<Mob>, With<Player>)>
) {
    for (_, transform, mut mob) in mobs.iter_mut() {
        let mut max_distance_squared: f32 = f32::MAX;
        let mut force = Vec3::default();

        for (_, p_transform, mut player) in players.iter_mut() {
            let distance = (p_transform.translation - transform.translation).length_squared();

            if distance < max_distance_squared {
                max_distance_squared = distance;
                force = p_transform.translation - transform.translation;
            }
        }

        mob.force += force;
    }
}

fn projectile_update(
    mut commands: Commands,
    time: Res<Time>,
    mut mobs: Query<(Entity, &mut Transform, &mut Mob), With<Mob>>,
    mut proj: Query<(Entity, &mut Transform, &mut Projectile), (With<Projectile>, Without<Mob>)>,
) {
    let dt = time.delta_seconds();

    // Handle the projectile interaction with the mobs.
    // If projectiles need to be de-spawned after the force analysis they can be added to the hashset.
    let mut despawns = HashSet::new();

    for (entity, mut transform, mut projectile) in proj.iter_mut() {
        projectile.lifetime.tick(time.delta());
        if projectile.lifetime.finished() {
            despawns.insert(entity);
        }


        let mut proj_accel = Vec3::default();
        for (_, transform1, mut mob1) in mobs.iter_mut() {
            let distance = transform1.translation - transform.translation;
            if distance.length_squared() < 0.5 {
                despawns.insert(entity);
                mob1.damage(projectile.damage);
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
        let color = get_mob_type(strength);

        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(color.into()),
            transform: Transform::from_translation(position),
            ..default()
        }).insert(Mob {
            pos: position,
            vel: Vec3::default(),
            acc: Vec3::default(),
            force: Vec3::default(),
            health: 10.0,
            strength,
            mass: 1.0,
            color: color,
            last_damaged: Stopwatch::default()
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

pub fn color_lerp(
    c1: Color,
    c2: Color,
    v: f32
) -> Color {
    return Color::rgb(
        c1.r() + (c2.r() - c1.r()) * v, c1.g() + (c2.g() - c1.g()) * v,
        c1.b() + (c2.b() - c1.b()) * v,
    )
}

