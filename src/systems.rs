use crate::components::*;
use bevy::ecs::schedule::*;
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

const TILE_SIZE: i32 = 64;
const WORLD_SIZE: i32 = 16;

fn get_nearest_tile_on_grid(x: f32, y: f32) -> (i32, i32) {
    ((x as i32 / TILE_SIZE), (y as i32 / TILE_SIZE))
}

fn is_position_in_bounds(x_or_y: f32) -> bool {
    let world_size_pixels = TILE_SIZE as f32 * WORLD_SIZE as f32;
    x_or_y < world_size_pixels && x_or_y > 0.
}

pub fn generate_collision_map(
    mut tile_map: ResMut<TileMap>,
    wall_query: Query<&Transform, Added<Wall>>,
    climbable_query: Query<&Transform, Added<Climbable>>,
) {
    for wall_transform in wall_query.iter() {
        tile_map.0.insert(
            get_nearest_tile_on_grid(wall_transform.translation.x, wall_transform.translation.y),
            TileType::Wall,
        );
    }
    for climbable_transform in climbable_query.iter() {
        tile_map.0.insert(
            get_nearest_tile_on_grid(
                climbable_transform.translation.x,
                climbable_transform.translation.y,
            ),
            TileType::Ladder,
        );
    }
}

pub fn movement(
    input: Res<Input<KeyCode>>,
    tile_map: Res<TileMap>,
    mut turn_state: ResMut<TurnState>,
    mut player_query: Query<(&Speed, &mut Transform), With<Player>>,
) {
    turn_state.player_just_took_turn = false;

    if let Ok((speed, mut transform)) = player_query.get_single_mut() {
        let mut direction = (0.0, 0.0);
        if input.just_pressed(KeyCode::A) || input.just_pressed(KeyCode::Left) {
            direction.0 -= 1.0;
        }
        if input.just_pressed(KeyCode::D) || input.just_pressed(KeyCode::Right) {
            direction.0 += 1.0;
        }
        if input.just_pressed(KeyCode::W) || input.just_pressed(KeyCode::Up) {
            direction.1 += 1.0;
        }
        if input.just_pressed(KeyCode::S) || input.just_pressed(KeyCode::Down) {
            direction.1 -= 1.0;
        }

        let current_position = &transform.translation;
        let mut new_position = current_position.clone();
        new_position.x += TILE_SIZE as f32 * direction.0 * speed.0;
        new_position.y += TILE_SIZE as f32 * direction.1 * speed.0;

        let going_down_while_falling = direction.1 < 0. && turn_state.player_is_falling;
        let mut new_position_is_valid = (
            true,
            new_position.y == current_position.y || going_down_while_falling,
        );
        if let Some(tile) = tile_map
            .0
            .get(&get_nearest_tile_on_grid(new_position.x, new_position.y))
        {
            match tile {
                TileType::Wall => {
                    new_position_is_valid.0 = false;
                }
                TileType::Ladder => {
                    new_position_is_valid.1 = true;
                }
            }
        }

        if is_position_in_bounds(new_position.x)
            && new_position != *current_position
            && new_position_is_valid.0
            && new_position_is_valid.1
        {
            transform.translation = new_position;
            turn_state.player_just_took_turn = true;

            // Apply gravity
            gravity(tile_map, player_query)
        }
    }
}

pub fn gravity(
    tile_map: Res<TileMap>,
    mut turn_state: ResMut<TurnState>,
    mut player_query: Query<(&Speed, &mut Transform), With<Player>>,
) {
    if let Ok((speed, mut transform)) = player_query.get_single_mut() {
        let current_position = &transform.translation;
        let mut tile_under_player =
            get_nearest_tile_on_grid(current_position.x, current_position.y);
        tile_under_player.1 -= 1;

        turn_state.player_is_falling = match tile_map.0.get(&tile_under_player) {
            Some(_) => false,
            None => true,
        };

        if turn_state.player_is_falling {
            let mut new_position = current_position.clone();
            new_position.y -= TILE_SIZE as f32 * speed.0;

            // TODO: trigger death when player reaches the edge of the level

            transform.translation = new_position;
        }
    }
}

pub fn run_if_player_moved(turn_state: Res<TurnState>) -> ShouldRun {
    if turn_state.player_just_took_turn {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn update_world() {
    println!("update_world");
}

const ASPECT_RATIO: f32 = 16.0 / 9.0;

pub fn camera_fit_inside_current_level(
    mut camera_query: Query<
        (
            &mut bevy::render::camera::OrthographicProjection,
            &mut Transform,
        ),
        Without<Player>,
    >,
    player_query: Query<&Transform, With<Player>>,
    level_query: Query<
        (&Transform, &Handle<LdtkLevel>),
        (Without<OrthographicProjection>, Without<Player>),
    >,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        let player_translation = player_translation.clone();

        let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

        for (level_transform, level_handle) in level_query.iter() {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let level = &ldtk_level.level;
                if level_selection.is_match(&0, &level) {
                    let level_ratio = level.px_wid as f32 / ldtk_level.level.px_hei as f32;

                    orthographic_projection.scaling_mode = bevy::render::camera::ScalingMode::None;
                    orthographic_projection.bottom = 0.0;
                    orthographic_projection.left = 0.0;
                    if level_ratio > ASPECT_RATIO {
                        // level is wider than the screen
                        orthographic_projection.top = (level.px_hei as f32 / 9.0).round() * 9.0;
                        orthographic_projection.right = orthographic_projection.top * ASPECT_RATIO;
                        camera_transform.translation.x = (player_translation.x
                            - level_transform.translation.x
                            - orthographic_projection.right / 2.0)
                            .clamp(0.0, level.px_wid as f32 - orthographic_projection.right);
                        camera_transform.translation.y = 0.0;
                    } else {
                        // level is taller than the screen
                        orthographic_projection.right = (level.px_wid as f32 / 16.0).round() * 16.0;
                        orthographic_projection.top = orthographic_projection.right / ASPECT_RATIO;
                        camera_transform.translation.y = (player_translation.y
                            - level_transform.translation.y
                            - orthographic_projection.top / 2.0)
                            .clamp(0.0, level.px_hei as f32 - orthographic_projection.top);
                        camera_transform.translation.x = 0.0;
                    }

                    camera_transform.translation.x += level_transform.translation.x;
                    camera_transform.translation.y += level_transform.translation.y;
                }
            }
        }
    }
}
