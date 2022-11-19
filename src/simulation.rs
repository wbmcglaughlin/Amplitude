use bevy::{
    prelude::*,
};
use crate::mob::Mob;

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
    time: Res<Time>
) {
    let dt = time.delta_seconds();

    if mobs.is_empty() {
        println!("Spawning Wave: {}", wave.current);
        let mobs = get_wave(wave.current);

        for mob_index in 0..mobs {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(0.0, 0.5,mob_index as f32 * 2.0),
                ..default()
            }).insert(Mob {
                vel: Vec3::default(),
                acc: Vec3::default(),
                health: 10.0,
                strength: 0.0
            });
        }
    } else {
        for (entity, mut transform, mut mob) in mobs.iter_mut() {
            mob.update(dt);

            transform.translation += mob.vel * dt;

            if mob.health <= 0.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn get_wave(
    wave: usize
) -> usize {
    return 5;
}

