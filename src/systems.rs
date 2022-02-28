use crate::components::*;
use bevy::ecs::schedule::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

const SPEED_BUTTON_LABEL: &str = "SPEED";
const STRENGTH_BUTTON_LABEL: &str = "STRENGTH";
const HEALTH_BUTTON_LABEL: &str = "HEALTH";

pub fn setup(asset_server: Res<AssetServer>) {
    asset_server.watch_for_changes().unwrap();
}

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(60.0)),
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(50.0)),
                    ..Default::default()
                },
                text: Text::with_section(
                    "CHOOSE AN UNFAIR ADVANTAGE",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 50.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });

            for label in vec![
                SPEED_BUTTON_LABEL,
                STRENGTH_BUTTON_LABEL,
                HEALTH_BUTTON_LABEL,
            ] {
                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                            margin: Rect::all(Val::Auto),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                label,
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 32.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        });
                    });
            }
        });
}

pub fn close_menu(mut commands: Commands, entity_query: Query<Entity>) {
    for e in entity_query.iter() {
        commands.entity(e).despawn();
    }
}

pub fn handle_ui_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<State<AppState>>,
    mut game_state: ResMut<GameState>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                match text.sections[0].value.as_str() {
                    SPEED_BUTTON_LABEL => {
                        println!("player chose speed");
                        game_state.player_advantage = Some(Advantage::Speed);
                    }
                    STRENGTH_BUTTON_LABEL => {
                        println!("player chose strength");
                        game_state.player_advantage = Some(Advantage::Strength);
                    }
                    HEALTH_BUTTON_LABEL => {
                        println!("player chose health");
                        game_state.player_advantage = Some(Advantage::Health);
                    }
                    _ => panic!("unknown button"),
                }

                app_state.set(AppState::InGame).unwrap();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

const TILE_SIZE: i32 = 64;
const WORLD_SIZE: i32 = 16;

pub fn setup_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("default.ldtk"),
        ..Default::default()
    });
}

pub fn apply_player_advantage(
    mut player_query: Query<(&mut Speed, &mut Damage, &mut Health), (With<Player>, Added<Player>)>,
    game_state: Res<GameState>,
) {
    if let Ok((mut speed, mut damage, mut health)) = player_query.get_single_mut() {
        match game_state.player_advantage {
            Some(Advantage::Speed) => speed.0 = 2,
            Some(Advantage::Strength) => damage.0 = 100,
            Some(Advantage::Health) => health.0 = 1000,
            None => panic!("no advantage was selected"),
        }
    }
}

pub fn teardown_world(mut commands: Commands, entity_query: Query<Entity>) {
    for e in entity_query.iter() {
        commands.entity(e).despawn();
    }
}

fn get_nearest_tile_on_grid(x: f32, y: f32) -> (i32, i32) {
    ((x as i32 / TILE_SIZE), (y as i32 / TILE_SIZE))
}

fn is_position_in_bounds(x_or_y: f32) -> bool {
    let world_size_pixels = TILE_SIZE as f32 * WORLD_SIZE as f32;
    x_or_y < world_size_pixels && x_or_y > 0.
}

pub fn generate_collision_map(
    mut tile_map: ResMut<TileMap>,
    wall_query: Query<&Transform, Added<WallTile>>,
    climbable_query: Query<&Transform, Added<ClimbableTile>>,
    falling_ice_query: Query<&Transform, Added<FallingIceTile>>,
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

    // Add entities for falling ice
    // They'll start moving once play is underneath
    for transform in falling_ice_query.iter() {
        tile_map.0.insert(
            get_nearest_tile_on_grid(transform.translation.x, transform.translation.y),
            TileType::FallingIce,
        );
    }
}

pub fn movement(
    input: Res<Input<KeyCode>>,
    tile_map: Res<TileMap>,
    mut game_state: ResMut<GameState>,
    mut player_query: Query<(&Speed, &mut Health, &mut Transform), With<Player>>,
) {
    game_state.player_just_took_turn = false;

    if let Ok((player_speed, mut player_health, mut player_transform)) =
        player_query.get_single_mut()
    {
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

        let current_position = &player_transform.translation;
        let mut new_position = current_position.clone();
        new_position.x += TILE_SIZE as f32 * direction.0;
        new_position.y += TILE_SIZE as f32 * direction.1;

        let going_down_while_falling = direction.1 < 0. && game_state.player_is_falling;
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
                TileType::Ladder | TileType::FallingIce => {
                    new_position_is_valid.1 = true;
                }
            }
        }

        if is_position_in_bounds(new_position.x)
            && new_position != *current_position
            && new_position_is_valid.0
            && new_position_is_valid.1
        {
            player_transform.translation = new_position;

            game_state.player_num_actions_taken += 1;
            if game_state.player_num_actions_taken % player_speed.0 as u32 == 0 {
                game_state.player_just_took_turn = true;
            }

            // Apply gravity
            apply_gravity(
                &tile_map,
                &mut game_state,
                &mut player_transform,
                &mut player_health,
            );
        }
    }
}

pub fn check_for_player_death(
    mut tile_map: ResMut<TileMap>,
    mut app_state: ResMut<State<AppState>>,
    mut game_state: ResMut<GameState>,
    player_query: Query<&Health, (With<Player>, Changed<Health>)>,
) {
    if let Ok(player_health) = player_query.get_single() {
        if player_health.0 <= 0 {
            tile_map.0.clear();
            *game_state = GameState::default();
            (*app_state).set(AppState::MainMenu).unwrap();
        }
    }
}

fn apply_gravity(
    tile_map: &Res<TileMap>,
    game_state: &mut ResMut<GameState>,
    mut player_transform: &mut Transform,
    mut player_health: &mut Health,
) {
    let current_position = &player_transform.translation;
    let mut tile_under_player = get_nearest_tile_on_grid(current_position.x, current_position.y);
    tile_under_player.1 -= 1;

    game_state.player_is_falling = match tile_map.0.get(&tile_under_player) {
        Some(_) => false,
        None => true,
    };

    if game_state.player_is_falling {
        let mut new_position = current_position.clone();
        new_position.y -= TILE_SIZE as f32;

        player_transform.translation = new_position;
        if !is_position_in_bounds(new_position.y) {
            player_health.0 = 0; // the player has fallen to their death
        }
    }
}
pub fn run_if_player_turn_over(game_state: Res<GameState>) -> ShouldRun {
    if game_state.player_just_took_turn {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn entities_are_overlapping(t1: &Transform, t2: &Transform) -> bool {
    let t1_on_grid = get_nearest_tile_on_grid(t1.translation.x, t1.translation.y);
    let t2_on_grid = get_nearest_tile_on_grid(t2.translation.x, t2.translation.y);
    t1_on_grid.0 == t2_on_grid.0 && t1_on_grid.1 == t2_on_grid.1
}

pub fn update_world(
    mut commands: Commands,
    mut player_query: Query<(&mut Health, &Damage, &Transform), With<Player>>,
    mut obstacle_query: Query<(Entity, &mut Health, &Damage, &Transform), Without<Player>>,
) {
    println!("update_world");

    if let Ok((mut player_health, player_damage, player_transform)) = player_query.get_single_mut()
    {
        for (obstacle_entity, mut obstacle_health, obstacle_damage, obstacle_transform) in
            obstacle_query.iter_mut()
        {
            if entities_are_overlapping(player_transform, obstacle_transform) {
                if player_damage.0 > 0 && obstacle_health.0 > 0 {
                    println!("player dealt {} damage to an obstacle", player_damage.0);
                    obstacle_health.0 -= player_damage.0;
                    if obstacle_health.0 <= 0 {
                        commands.entity(obstacle_entity).despawn();
                    }
                }

                if obstacle_damage.0 > 0 && player_health.0 > 0 {
                    println!("obstacle dealt {} damage to the player", obstacle_damage.0);
                    player_health.0 -= obstacle_damage.0;
                }
            }
        }
    }
}

pub fn update_falling_ice(tile_map: Res<TileMap>, player_query: Query<&Transform, With<Player>>) {
    let transform = player_query.single();
    let (x, y) = get_nearest_tile_on_grid(transform.translation.x, transform.translation.y);

    for j in y..WORLD_SIZE {
        let tile_to_inspect = (x, j);
        match tile_map.0.get(&tile_to_inspect) {
            Some(TileType::Wall) => {
                println!("Found wall!");
                break;
            }
            Some(TileType::FallingIce) => {
                println!("Found falling ice!");
                break;
            }
            Some(TileType::Ladder) => {} // go through
            None => {}                   // keep going
        }
    }
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
