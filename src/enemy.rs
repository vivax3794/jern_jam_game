use bevy::prelude::*;
use bevy_prototype_debug_lines::*;

use noise::{Fbm, NoiseFn, Seedable};
use rand::{prelude::*};

pub struct EnemyPlugin;

const PATH_LENGTH: usize = 30;

const MIN_SCALE: f32 = 0.4;
const MAX_SCALE: f32 = 1.2;
const START_HEALTH: f32 = 1000.0;
const SPEED: f32 = 100.0;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_spawner);
        app.add_system(move_upwards);
        app.add_system(spawn_enemies);
        app.add_system(debug_draw_path);
        app.add_system(enemy_death);
        app.add_system(set_size_to_health);
        app.insert_resource(EnemySpawnTimer(Timer::from_seconds(1.5, true)));
        app.insert_resource(EnemyPath(create_starting_path()));
    }
}

fn create_starting_path() -> Vec<Vec2> {
    let mut path = Vec::with_capacity(PATH_LENGTH);

    let mut rng = rand::thread_rng();
    let perlin_x = Fbm::new().set_seed(rng.next_u32());
    let perlin_y = Fbm::new().set_seed(rng.next_u32());

    let mut max_x: f32 = 0.0;
    let mut max_y: f32 = 0.0;

    for i in 0..PATH_LENGTH {
        let i = (i as f64) / 20.0 + 1.0;
        let x = perlin_x.get([i, 0.1]) as f32;
        let y = perlin_y.get([i, 0.1]) as f32;

        path.push(Vec2::new(x, y));

        max_x = max_x.max(x.abs());
        max_y = max_y.max(y.abs());
    }

    let scale_x = 400.0 / max_x;
    let scale_y = 290.0 / max_y;

    let path: Vec<Vec2> = path.into_iter().map(|vec| {
        Vec2::new(vec.x * scale_x, vec.y * scale_y)
    }).collect();

    path
}

fn setup_spawner(mut commands: Commands, path: Res<EnemyPath>) {
    commands.spawn().insert(EnemySpawner(Transform::from_translation(path.0[0].extend(0.0))));
}

#[derive(Component)]
pub struct Enemy;

/// Holds the index into the path
#[derive(Component)]
pub struct PathMover(usize);

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Bundle)]
pub struct EnemyBundle {
    _m: Enemy,

    #[bundle]
    sprite: SpriteBundle,
    path: PathMover,
    health: Health,
}

impl EnemyBundle {
    fn new_at(mut location: Transform, asset: &Res<AssetServer>) -> Self {
        location.scale = Vec3::new(0.6, 0.6, 1.0);
        EnemyBundle {
            _m: Enemy,

            sprite: SpriteBundle {
                transform: location,
                texture: asset.load("enemy.png"),
                ..default()
            },
            path: PathMover(0),
            health: Health(START_HEALTH),
        }
    }
}

#[derive(Component)]
struct EnemySpawner(Transform);

struct EnemySpawnTimer(Timer);

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    query: Query<&EnemySpawner>,
    asset: Res<AssetServer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for spawner in query.iter() {
            commands.spawn_bundle(EnemyBundle::new_at(spawner.0, &asset));
        }
    }
}

pub struct EnemyPath(pub Vec<Vec2>);

fn debug_draw_path(path: Res<EnemyPath>, mut lines: ResMut<DebugLines>) {
    let mut last_point = path.0[0];
    for point in path.0.iter().skip(1) {
        lines.line(last_point.extend(0.), point.extend(0.), 0.);
        last_point = *point;
    }
}

fn move_upwards(
    mut commands: Commands,
    time: Res<Time>,
    path: Res<EnemyPath>,
    mut query: Query<(Entity, &mut Transform, &mut PathMover), With<Enemy>>,
) {
    for (entity, mut trans, mut path_index) in query.iter_mut() {
        let target_point = path.0[path_index.0];
        let pos = trans.translation.truncate();

        if pos.distance(target_point) <= 1. {
            path_index.0 += 1;

            if path_index.0 == path.0.len() {
                // Hit the end, time to despawn!
                // TODO: Remove health from player or something like that!
                commands.entity(entity).despawn();
            }
        } else {
            let direction = (target_point - pos).normalize_or_zero();
            trans.translation += (direction * SPEED * time.delta_seconds()).extend(0.);

            let angle = Vec2::new(-1.0, 0.0).angle_between(direction);
            trans.rotation = Quat::from_rotation_z(angle);
        }
    }
}

fn enemy_death(mut commands: Commands, query: Query<(Entity, &Health), With<Enemy>>) {
    query.for_each(|(entity, health)| {
        if health.0 <= 0. {
            commands.entity(entity).despawn();
        }
    })
}

fn set_size_to_health(mut query: Query<(&mut Transform, &Health), With<Enemy>>) {
    query.for_each_mut(|(mut trans, health)| {
        let scale = health.0 / (START_HEALTH / (MAX_SCALE - MIN_SCALE)) + MIN_SCALE;

        // Z-scale has no effect with our camera, so leave it at 1
        trans.scale = Vec3::new(scale, scale, 1.);
    })
}
