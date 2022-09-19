use bevy::prelude::*;
use bevy::winit::WinitSettings;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_prototype_lyon::prelude::ShapePlugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(WinitSettings::game())
        .insert_resource(WindowDescriptor {
            title: "Tower Game".to_owned(),
            width: 1000.,
            height: 600.,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)));

    app
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(DebugLinesPlugin::default())
        .add_startup_system(setup);
    app.add_plugin(tower_defence_game::GamePlugin);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
