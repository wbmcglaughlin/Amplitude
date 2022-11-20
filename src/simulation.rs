use bevy::{
    prelude::*,
};
use rand::{Rng, thread_rng};
use crate::mob::Mob;
use crate::system_adapter::new;

pub struct SimulationPlugin;
impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Wave {
            current: 4
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
    time: Res<Time>
) {
    let dt = time.delta_seconds();
    let mut prng = thread_rng();

    if mobs.is_empty() {
        wave.current += 1;
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
                strength: prng.gen::<f32>()
            });
        }
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

                    // Attraction Force
                    force += distance;

                    // Repel Force
                    let mut repel = 0.1 / distance;
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
                if new_position.distance_squared(position.clone()) < 0.5 * 0.5 {

                }
            }

            transform.translation += new_distance;

            if mob.health <= 0.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn get_wave(
    wave: usize
) -> usize {
    return wave;
}

