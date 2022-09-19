use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

use crate::enemy::{self, Enemy};

const Z_INDEX: f32 = 100.;
const DPS: f32 = 300.0;
const RANGE: f32 = 200.0;

#[derive(Component)]
struct Tower;

#[derive(Bundle)]
struct TowerBundle {
    _m: Tower,

    #[bundle]
    sprite: ShapeBundle,
}

impl TowerBundle {
    pub fn new_at(location: Transform) -> Self {
        TowerBundle {
            _m: Tower,

            sprite: GeometryBuilder::build_as(
                &shapes::Circle {
                    radius: 10.,
                    ..default()
                },
                DrawMode::Fill(FillMode::color(Color::BLUE)),
                location,
            ),
        }
    }
}

pub struct TowerPlugin;
impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_starting_towers);
        app.add_system(target_closest_enemy);
    }
}

fn create_starting_towers(mut commands: Commands) {
    // commands.spawn_bundle(TowerBundle::new_at(Transform::from_xyz(50., 0., Z_INDEX)));
    // commands.spawn_bundle(TowerBundle::new_at(Transform::from_xyz(-150., 150., Z_INDEX)));
    // commands.spawn_bundle(TowerBundle::new_at(Transform::from_xyz(-30., -250., Z_INDEX)));
}

fn target_closest_enemy(
    time: Res<Time>,
    mut lines: ResMut<DebugLines>,
    towers: Query<&Transform, With<Tower>>,
    mut enemies: Query<(&Transform, &mut enemy::Health), With<Enemy>>,
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
