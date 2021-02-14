use bevy::prelude::*;

fn main() {
    App::build()
        .add_system(move_max.system())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("sprites/sprite_max.png");
    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        })
        .with(Max);
}

struct Max;

fn move_max(
    keyboard_input: Res<Input<KeyCode>>,
    mut max_positions: Query<&mut Transform, With<Max>>,
) {
    for mut transform in max_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 1.0;
        }
    }
}