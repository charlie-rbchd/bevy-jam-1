use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod components;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(components::TileMap::default())
        .insert_resource(components::TurnState {
            player_just_took_turn: false,
        })
        .insert_resource(LdtkSettings {
            load_level_neighbors: true,
            use_level_world_translations: true,
            ..Default::default()
        })
        .add_startup_system(systems::setup)
        .add_system(systems::camera_fit_inside_current_level)
        .add_system(systems::generate_collision_map)
        .add_system(systems::movement)
        .add_system(systems::update_world)
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .register_ldtk_int_cell::<components::LadderBundle>(2)
        .register_ldtk_entity::<components::PlayerBundle>("Player")
        .run();
}
