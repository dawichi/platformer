use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod components;
pub use components::*;

mod camera;
pub use camera::*;

mod player;
pub use player::*;

mod map;
pub use map::*;

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
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(setup.system())
        .add_startup_stage("floor_setup", SystemStage::single(spawn_floor.system()))
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(new_camera_2d());
    commands.insert_resource(Materials {
        player_material: materials.add(Color::rgb(0.9, 0.7, 0.6).into()),
        floor_material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
    });
}
