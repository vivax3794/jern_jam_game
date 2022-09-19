use bevy::prelude::*;
use bevy_prototype_debug_lines::*;

// use rand::{prelude::*, distributions::Uniform};
use noise::{NoiseFn, Seedable, Perlin, OpenSimplex, SuperSimplex, Fbm};

pub struct EnemyPlugin;

const PATH_LENGTH: usize = 30;
const PATH_X_SIDES: f32 = 300.;
const PATH_Y_SIDES: f32 = 300.;

const MIN_SCALE: f32 = 0.4;
const MAX_SCALE: f32 = 1.2;
const START_HEALTH: f32 = 1000.0;


impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
        app.add_startup_system(create_starting_path);
        app.add_system(move_upwards);
        app.add_system(spawn_enemies);
        app.add_system(debug_draw_path);
        app.add_system(enemy_death);
        app.add_system(set_size_to_health);
        app.insert_resource(EnemySpawnTimer(Timer::from_seconds(1.5, true)));
    }
}

fn create_starting_path(mut commands: Commands) {
    let mut path = Vec::with_capacity(PATH_LENGTH);
    // path.push(Vec2::new(0.0, -PATH_Y_SIDES));

    // let mut rng = rand::thread_rng();
    // let range_x = Uniform::new(-PATH_X_SIDES, PATH_X_SIDES);
    // let range_y = Uniform::new(-PATH_Y_SIDES, PATH_Y_SIDES);

    // for _ in 0..PATH_LENGTH {
    //     let pos = Vec2::new(
    //         rng.sample(range_x),
    //         rng.sample(range_y)
    //     );
    //     path.push(pos);
    // }

    // path.push(Vec2::new(0.0, PATH_Y_SIDES));
    
    let perlin_x = Fbm::new().set_seed(374);
    let perlin_y = Fbm::new().set_seed(4069);

    for i in 0..PATH_LENGTH {
        let i = (i as f64) / 20.0;
        let x = perlin_x.get([i ,0.1]) as f32 * PATH_X_SIDES * 2.;
        let y = perlin_y.get([i, 0.1]) as f32 * PATH_Y_SIDES * 2.;

        println!("{i}: {x}, {y}");

        path.push(Vec2::new(x, y));
    }

    commands.insert_resource(EnemyPath(path));


    
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

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(EnemySpawner(Transform::from_xyz(0., -300., 0.)));
}

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

pub struct EnemyPath(Vec<Vec2>);

fn debug_draw_path(path: Res<EnemyPath>, mut lines: ResMut<DebugLines>) {
    let mut last_point = Vec2::new(0., -300.);
    for point in path.0.iter() {
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
            trans.translation += (direction * 50.0 * time.delta_seconds()).extend(0.);

            let angle = -direction.angle_between(Vec2::new(0.0, 1.0));
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