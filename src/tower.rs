use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

use crate::enemy::{self, Enemy, EnemyPath};

use rand::prelude::*;

const Z_INDEX: f32 = 100.;
const DPS: f32 = 100.0;
const RANGE: f32 = 150.0;

const TOWER_VARIANCE_RANGE: std::ops::Range<f32> = 50.0..100.0;

#[derive(Component)]
struct Tower;

#[derive(Bundle)]
struct TowerBundle {
    _m: Tower,

    #[bundle]
    sprite: ShapeBundle,
}

impl TowerBundle {
    pub fn new_at(location: Vec2) -> Self {
        TowerBundle {
            _m: Tower,

            sprite: GeometryBuilder::build_as(
                &shapes::Circle {
                    radius: 10.,
                    ..default()
                },
                DrawMode::Fill(FillMode::color(Color::BLUE)),
                Transform::from_translation(location.extend(Z_INDEX)),
            ),
        }
    }
}

pub struct TowerPlugin;
impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_starting_towers);
        app.add_system(add_tower_on_click);
        app.add_system(target_closest_enemy);
    }
}

fn add_tower_on_click(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    query: Query<(&Camera, &GlobalTransform)>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();

        if let Some(position) = window.cursor_position() {
            // we assume there is only ever one camera
            let (camera, camera_transform) = query.single();

            let window_size = Vec2::new(
                window.width() as f32,
                window.height() as f32,
            );
            let gpu_cords = (position / window_size) * 2.0 - Vec2::ONE;
            let gpu_to_world_matrix = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
            
            let world_pos = gpu_to_world_matrix.project_point3(gpu_cords.extend(0.0));
            let world_pos = world_pos.truncate();

            // Spawn tower at click location!
            commands.spawn_bundle(TowerBundle::new_at(world_pos));
        }
    }
}

fn create_starting_towers(mut commands: Commands, enemy_path: Res<EnemyPath>) {
    let mut rng = rand::thread_rng();
    
    for _ in 0..5 {
        let point = enemy_path.0.choose(&mut rng).unwrap();
        
        let variance_length = rng.gen_range(TOWER_VARIANCE_RANGE);
        let variance_angle = rng.gen_range(0.0..(2.0 * std::f32::consts::PI));
        let variance = Vec2::from_angle(variance_angle) * variance_length;

        let pos = *point + variance;
        
        commands.spawn_bundle(TowerBundle::new_at(pos));
    }
}

fn target_closest_enemy(
    time: Res<Time>,
    mut lines: ResMut<DebugLines>,
    towers: Query<&Transform, With<Tower>>,
    mut enemies: Query<(&Transform, &mut enemy::Health), With<Enemy>>,g
) {
    for tower in towers.iter() {
        let closest = enemies.iter_mut().min_by(|(a, _), (b, _)| {
            let a = a.translation.distance(tower.translation);
            let b = b.translation.distance(tower.translation);
            a.partial_cmp(&b).unwrap()
        });

        if let Some((trans, mut health)) = closest {
            if trans.translation.distance(tower.translation) <= RANGE {
                // DEBUG LINE
                lines.line_colored(tower.translation, trans.translation, 0., Color::RED);

                health.0 -= time.delta_seconds() * DPS;
            }
        }
    }
}
