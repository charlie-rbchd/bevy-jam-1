use bevy::ecs::schedule::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod components;
mod systems;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum SystemOrder {
    WorldGeneration,
    InputHandling,
    WorldTick,
    Camera,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(components::TileMap::default())
        .insert_resource(components::TurnState {
            player_just_took_turn: false,
            player_is_falling: false,
        })
        .insert_resource(LdtkSettings {
            load_level_neighbors: true,
            use_level_world_translations: true,
            ..Default::default()
        })
        .insert_resource(ReportExecutionOrderAmbiguities)
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .register_ldtk_int_cell::<components::LadderBundle>(2)
        .register_ldtk_entity::<components::PlayerBundle>("Player")
        .register_ldtk_entity::<components::ObstacleBundle>("Obstacle")
        .add_startup_system(systems::setup)
        .add_system_set(
            SystemSet::new()
                .label(SystemOrder::WorldGeneration)
                .with_system(systems::generate_collision_map),
        )
        .add_system_set(
            SystemSet::new()
                .label(SystemOrder::InputHandling)
                .after(SystemOrder::WorldGeneration)
                .with_system(systems::movement),
        )
        .add_system_set(
            SystemSet::new()
                .label(SystemOrder::Camera)
                .after(SystemOrder::InputHandling)
                .with_system(systems::camera_fit_inside_current_level),
        )
        .add_system_set(
            SystemSet::new()
                .label(SystemOrder::WorldTick)
                .after(SystemOrder::InputHandling)
                .with_run_criteria(systems::run_if_player_moved)
                .with_system(systems::update_world),
        )
        .run();
}
