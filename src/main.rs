use bevy::ecs::schedule::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::AudioPlugin;

mod components;
mod systems;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum GameSystem {
    LoadWorld,
    BuildTilemap,
    ApplyPlayerAdvantage,
    MovePlayer,
    ApplyPlayerVisualEffects,
    CheckForExitStates,
    ApplyDamageToPlayer,
    SpawnFallingIceOverPlayer,
    MoveFallingIce,
    FitCamera,
    TeardownWorld,
    _SetupMenu,
    _CloseMenu,
}

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .insert_resource(ClearColor(Color::rgb_u8(174, 188, 233)))
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_plugin(AudioPlugin)
        .insert_resource(LevelSelection::Index(0))
        .insert_resource(components::TileMap::default())
        .insert_resource(components::GameState::default())
        // .insert_resource(ReportExecutionOrderAmbiguities)
        .add_state(components::AppState::MainMenu)
        .register_ldtk_int_cell::<components::WallTileBundle>(1)
        .register_ldtk_int_cell::<components::ClimableTileBundle>(2)
        .register_ldtk_int_cell::<components::FallingIceTileBundle>(3)
        .register_ldtk_entity::<components::PlayerBundle>("Player")
        .register_ldtk_entity::<components::ObstacleSpikeBundle>("ObstacleSpike")
        .register_ldtk_entity::<components::ObstacleBlockBundle>("ObstacleBlock")
        .register_ldtk_entity::<components::FallingIceBundle>("FallingIce")
        .register_ldtk_entity::<components::GoalBundle>("Goal")
        .add_startup_system(systems::setup)
        .add_system_set(
            SystemSet::on_enter(components::AppState::InGame)
                .label(GameSystem::LoadWorld)
                .with_system(systems::load_world),
        )
        .add_system_set(
            SystemSet::on_update(components::AppState::InGame)
                .label(GameSystem::BuildTilemap)
                .with_system(systems::build_tilemap_with_added_tiles),
        )
        .add_system_set(
            SystemSet::on_update(components::AppState::InGame)
                .label(GameSystem::ApplyPlayerAdvantage)
                .with_system(systems::apply_player_advantage_on_player_added),
        )
        .add_system_set(
            SystemSet::on_update(components::AppState::InGame)
                .label(GameSystem::MovePlayer)
                .with_system(systems::move_player_from_input),
        )
        .add_system_set(
            SystemSet::on_update(components::AppState::InGame)
                .after(GameSystem::MovePlayer)
                .label(GameSystem::ApplyPlayerVisualEffects)
                .with_system(systems::apply_player_visual_effects),
        )
        .add_system_set(
            SystemSet::on_update(components::AppState::InGame)
                .label(GameSystem::FitCamera)
                .with_system(systems::fit_camera_inside_current_level),
        )
        .add_system_set(
            SystemSet::on_update(components::AppState::InGame)
                .with_run_criteria(systems::run_if_world_should_update)
                .after(GameSystem::MovePlayer)
                .label(GameSystem::MoveFallingIce)
                .with_system(systems::move_falling_ice),
        )
        .add_system_set(
            SystemSet::on_update(components::AppState::InGame)
                .with_run_criteria(systems::run_if_world_should_update)
                .after(GameSystem::MoveFallingIce)
                .label(GameSystem::ApplyDamageToPlayer)
                .with_system(systems::apply_damage_to_player),
        )
        .add_system_set(
            SystemSet::on_update(components::AppState::InGame)
                .with_run_criteria(systems::run_if_world_should_update)
                .after(GameSystem::MoveFallingIce)
                .label(GameSystem::SpawnFallingIceOverPlayer)
                .with_system(systems::spawn_falling_ice_over_player), // spawn ice now for the next turn
        )
        .add_system_set(
            SystemSet::on_update(components::AppState::InGame)
                .after(GameSystem::MovePlayer)
                .label(GameSystem::CheckForExitStates)
                .with_system(systems::check_for_player_death)
                .with_system(systems::check_player_reached_goal)
                .with_system(systems::exit_on_esc),
        )
        .add_system_set(
            SystemSet::on_exit(components::AppState::InGame)
                .label(GameSystem::TeardownWorld)
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
