use bevy::prelude::*;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ResourceAmount(1000));
        app.add_startup_system(create_text);
        app.add_system(update_score);   
    }
}

pub struct ResourceAmount(pub u32);

#[derive(Component)]
struct ResourceTextCounter;

fn create_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/DS-DIGI.ttf");
    let text_style = TextStyle {
        color: Color::BLUE,
        font_size: 30.0,
        font,
    };

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("NaN", text_style),
            transform: Transform::from_xyz(0.0, 300.0, 200.0),
            ..default()
        })
        .insert(ResourceTextCounter);
}

fn update_score(
    resource_amount: Res<ResourceAmount>,
    mut query: Query<&mut Text, With<ResourceTextCounter>>,
) {
    if resource_amount.is_changed() {
        let mut text = query.single_mut();
        text.sections[0].value = resource_amount.0.to_string();
    }
}
