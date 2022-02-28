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
    WorldTeardown,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(components::TileMap::default())
        .insert_resource(components::TurnState {
            player_just_took_turn: false,
            player_num_actions_taken: 0,
            player_is_falling: false,
        })
        .insert_resource(LdtkSettings {
            load_level_neighbors: true,
            use_level_world_translations: true,
            ..Default::default()
        })
        // .insert_resource(ReportExecutionOrderAmbiguities)
        .add_state(components::AppState::MainMenu)
        .register_ldtk_int_cell::<components::WallTileBundle>(1)
        .register_ldtk_int_cell::<components::ClimableTileBundle>(2)
        .register_ldtk_int_cell::<components::FallingIceTileBundle>(3)
        .register_ldtk_entity::<components::PlayerBundle>("Player")
        .register_ldtk_entity::<components::ObstacleBundle>("Obstacle")
        .add_startup_system(systems::setup)
        .add_system_set(
            SystemSet::on_enter(components::AppState::InGame)
                .label(SystemOrder::WorldGeneration)
                .with_system(systems::setup_world),
        )
        .add_system_set(
            SystemSet::on_update(components::AppState::InGame)
                .label(SystemOrder::WorldGeneration)
                .with_system(systems::generate_collision_map),
        )
        .add_system_set(
            SystemSet::on_update(components::AppState::InGame)
                .label(SystemOrder::InputHandling)
                .after(SystemOrder::WorldGeneration)
                .with_system(systems::movement),
        )
        .add_system_set(
            SystemSet::on_update(components::AppState::InGame)
                .label(SystemOrder::Camera)
                .after(SystemOrder::InputHandling)
                .with_system(systems::camera_fit_inside_current_level),
        )
        .add_system_set(
            SystemSet::on_update(components::AppState::InGame)
                .label(SystemOrder::WorldTick)
                .after(SystemOrder::InputHandling)
                .with_run_criteria(systems::run_if_player_turn_over)
                .with_system(systems::check_for_player_death)
                .with_system(systems::update_world)
                .with_system(systems::update_falling_ice),
        )
        .add_system_set(
            SystemSet::on_exit(components::AppState::InGame)
                .label(SystemOrder::WorldTeardown)
                .with_system(systems::teardown_world),
        )
        .add_system_set(
            SystemSet::on_enter(components::AppState::MainMenu).with_system(systems::setup_menu),
        )
        .add_system_set(
            SystemSet::on_update(components::AppState::MainMenu)
                .with_system(systems::handle_ui_buttons),
        )
        .add_system_set(
            SystemSet::on_exit(components::AppState::MainMenu).with_system(systems::close_menu),
        )
        .run();
}
