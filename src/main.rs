use bevy::prelude::*;

mod camera;
pub use camera::*;

struct Player;

fn spawn_player(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.9, 0.7, 0.6).into()),
            sprite: Sprite::new(Vec2::new(1.0, 1.0)),
            ..Default::default()
        })
        .insert(Player);
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(new_camera_2d());
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Platformer Game".to_string(),
            width: 800.0,
            height: 600.0,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_startup_system(setup.system())
        .add_startup_stage("player_setup", SystemStage::single(spawn_player.system()))
        .add_plugins(DefaultPlugins)
        // .add_startup_system(setup.system())
        // .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}