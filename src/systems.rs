use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    asset_server.watch_for_changes().unwrap();

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("default.ldtk"),
        ..Default::default()
    });
}
