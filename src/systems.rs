use crate::components::*;
use bevy::ecs::schedule::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::Audio;
use rand::Rng;

const SPEED_BUTTON_LABEL: &str = "SPEED";
const STRENGTH_BUTTON_LABEL: &str = "STRENGTH";
const HEALTH_BUTTON_LABEL: &str = "HEALTH";

pub fn setup(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    asset_server.watch_for_changes().unwrap();
    audio.play_looped(asset_server.load("audio/AMB_PolarWind_Loop.ogg"));
}

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        color: Color::rgb_u8(174, 188, 233).into(),
        ..Default::default()
    });
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(75.0)),
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
                    "THE LAST CLIMB",
                    TextStyle {
                        font: asset_server.load("fonts/Minecraft.ttf"),
                        font_size: 62.0,
                        color: Color::rgb_u8(234, 237, 194),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });

            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(50.0)),
                    ..Default::default()
                },
                text: Text::with_section(
                    "CHOOSE AN UNFAIR ADVANTAGE",
                    TextStyle {
                        font: asset_server.load("fonts/Minecraft.ttf"),
                        font_size: 42.0,
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
                        color: Color::rgb_u8(116, 147, 226).into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                label,
                                TextStyle {
                                    font: asset_server.load("fonts/Minecraft.ttf"),
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

    // preload audio
    commands.insert_resource(UiSounds {
        button_clicked_sfx: asset_server.load("audio/SFX_PlayerClimb_02.ogg"),
    });
}

pub fn close_menu(mut commands: Commands, entity_query: Query<Entity>) {
    for e in entity_query.iter() {
        commands.entity(e).despawn();
    }
    // unload audio
    commands.remove_resource::<UiSounds>();
}

pub fn handle_ui_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<State<AppState>>,
    mut game_state: ResMut<GameState>,
    mut text_query: Query<&mut Text>,
    ui_sounds: Res<UiSounds>,
    audio: Res<Audio>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                audio.play(ui_sounds.button_clicked_sfx.clone());

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
                *color = Color::rgb_u8(193, 238, 247).into();
                text.sections[0].style.color = Color::rgb(0.3, 0.3, 0.3).into();
            }
            Interaction::None => {
                *color = Color::rgb_u8(116, 147, 226).into();
                text.sections[0].style.color = Color::rgb(0.9, 0.9, 0.9).into();
            }
        }
    }
}

const TILE_SIZE: i32 = 64;
const WORLD_SIZE: i32 = 16;

pub fn load_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("default_baked.ldtk"),
        ..Default::default()
    });
    // preload audio
    commands.insert_resource(GameSounds {
        player_movement_sfxs: vec![
            asset_server.load("audio/SFX_PlayerMovement_01.ogg"),
            asset_server.load("audio/SFX_PlayerMovement_02.ogg"),
            asset_server.load("audio/SFX_PlayerMovement_03.ogg"),
        ],
        player_climb_up_sfxs: vec![
            asset_server.load("audio/SFX_PlayerClimb_01.ogg"),
            asset_server.load("audio/SFX_PlayerClimb_02.ogg"),
        ],
        player_climb_down_sfxs: vec![
            asset_server.load("audio/SFX_PlayerClimb_03.ogg"),
            asset_server.load("audio/SFX_PlayerClimb_04.ogg"),
        ],
        player_hit_sfxs: vec![
            asset_server.load("audio/SFX_Hit_01.ogg"),
            asset_server.load("audio/SFX_Hit_02.ogg"),
        ],
        falling_ice_sfx: asset_server.load("audio/SFX_FallingIce.ogg"),
        goal_sfx: asset_server.load("audio/SFX_Goal.ogg"),
    });
}

pub fn teardown_world(mut commands: Commands, entity_query: Query<Entity>) {
    for e in entity_query.iter() {
        commands.entity(e).despawn();
    }
    // unload audio
    commands.remove_resource::<GameSounds>();
}

pub fn apply_player_advantage_on_player_added(
    mut player_query: Query<(&mut Speed, &mut Damage, &mut Health), (With<Player>, Added<Player>)>,
    game_state: Res<GameState>,
) {
    if let Ok((mut speed, mut damage, mut health)) = player_query.get_single_mut() {
        match game_state.player_advantage {
            Some(Advantage::Speed) => speed.0 = 2,
            Some(Advantage::Strength) => damage.0 = 100,
            Some(Advantage::Health) => health.0 = 200,
            None => panic!("no advantage was selected"),
        }
    }
}

fn get_nearest_tile_on_grid(x: f32, y: f32) -> (i32, i32) {
    ((x / TILE_SIZE as f32) as i32, (y / TILE_SIZE as f32) as i32)
}

fn is_position_in_bounds(x_or_y: f32) -> bool {
    let world_size_pixels = TILE_SIZE as f32 * WORLD_SIZE as f32;
    x_or_y < world_size_pixels && x_or_y > 0.
}

// Return top-left
fn tile_pos_to_sprite_pos(x: i32, y: i32) -> Vec3 {
    let size = TILE_SIZE as f32;
    let half = size / 2.;
    Vec3::new(x as f32 * size + half, y as f32 * size + half, 100.)
}

pub fn build_tilemap_with_added_tiles(
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

pub fn move_player_from_input(
    input: Res<Input<KeyCode>>,
    tile_map: Res<TileMap>,
    mut game_state: ResMut<GameState>,
    mut player_query: Query<(&Speed, &mut Health, &mut Transform), With<Player>>,
    game_sounds: Res<GameSounds>,
    audio: Res<Audio>,
) {
    game_state.player_just_took_turn = false;

    if let Ok((player_speed, mut player_health, mut player_transform)) =
        player_query.get_single_mut()
    {
        // Then move the player
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

        // reset falling state now that player moved (last frame)
        let current_position = player_transform.translation.clone();
        let mut tile_under_player =
            get_nearest_tile_on_grid(current_position.x, current_position.y);
        tile_under_player.1 -= 1;
        game_state.player_is_falling = match tile_map.0.get(&tile_under_player) {
            Some(_) => false,
            None => true,
        };

        let mut new_position = current_position.clone();
        new_position.x += TILE_SIZE as f32 * direction.0;
        new_position.y += TILE_SIZE as f32 * direction.1;

        let going_down_while_falling = direction.1 < 0. && game_state.player_is_falling;
        let mut new_position_is_valid = (
            true,
            new_position.y == current_position.y || going_down_while_falling,
        );
        direction.1 = 0.0; // gravity will take care of it

        // Fetch tile where the player wants to go
        if let Some(tile) = tile_map
            .0
            .get(&get_nearest_tile_on_grid(new_position.x, new_position.y))
        {
            // Fetch tile below this one
            let mut wall_is_under = false;
            if let Some(tile_below) = tile_map.0.get(&get_nearest_tile_on_grid(
                new_position.x,
                new_position.y - 1.0,
            )) {
                wall_is_under = match *tile_below {
                    TileType::Wall => true,
                    _ => false,
                };
            }

            match tile {
                TileType::Wall => {
                    new_position_is_valid.0 = wall_is_under;
                }
                TileType::Ladder | TileType::FallingIce => {
                    new_position_is_valid.1 = true;
                }
            }
        }

        if is_position_in_bounds(new_position.x)
            && new_position != current_position
            && new_position_is_valid.0
            && new_position_is_valid.1
        {
            game_state.player_previous_pos = player_transform.translation;
            player_transform.translation = new_position;

            game_state.player_num_actions_taken += 1;
            if game_state.player_num_actions_taken % player_speed.0 as u32 == 0 {
                game_state.player_just_took_turn = true;
            }

            apply_gravity(
                &tile_map,
                &mut game_state,
                &mut player_transform,
                &mut player_health,
            );

            let mut rng = rand::thread_rng();
            if !game_state.player_is_falling {
                if direction.1 > 0. {
                    audio.play(
                        game_sounds.player_climb_up_sfxs
                            [rng.gen_range(0..game_sounds.player_climb_up_sfxs.len())]
                        .clone(),
                    );
                } else if direction.1 < 0. {
                    audio.play(
                        game_sounds.player_climb_down_sfxs
                            [rng.gen_range(0..game_sounds.player_climb_down_sfxs.len())]
                        .clone(),
                    );
                } else {
                    audio.play(
                        game_sounds.player_movement_sfxs
                            [rng.gen_range(0..game_sounds.player_movement_sfxs.len())]
                        .clone(),
                    );
                }
            } else if player_health.0 <= 0 {
                audio.play(
                    game_sounds.player_hit_sfxs
                        [rng.gen_range(0..game_sounds.player_hit_sfxs.len())]
                    .clone(),
                );
            }
        }
    }
}

fn return_to_main_menu(
    tile_map: &mut ResMut<TileMap>,
    app_state: &mut ResMut<State<AppState>>,
    game_state: &mut ResMut<GameState>,
) {
    tile_map.0.clear();
    **game_state = GameState::default();
    (*app_state).set(AppState::MainMenu).unwrap();
}

pub fn check_for_player_death(
    mut tile_map: ResMut<TileMap>,
    mut app_state: ResMut<State<AppState>>,
    mut game_state: ResMut<GameState>,
    player_query: Query<&Health, (With<Player>, Changed<Health>)>,
) {
    if let Ok(player_health) = player_query.get_single() {
        if player_health.0 <= 0 {
            return_to_main_menu(&mut tile_map, &mut app_state, &mut game_state);
        }
    }
}

pub fn check_player_reached_goal(
    goal_query: Query<&Transform, With<Goal>>,
    player_query: Query<&Transform, (With<Player>, Changed<Transform>)>,
    mut tile_map: ResMut<TileMap>,
    mut app_state: ResMut<State<AppState>>,
    mut game_state: ResMut<GameState>,
    game_sounds: Res<GameSounds>,
    audio: Res<Audio>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(goal_transform) = goal_query.get_single() {
            if entities_are_overlapping(player_transform, goal_transform) {
                audio.play(game_sounds.goal_sfx.clone());
                return_to_main_menu(&mut tile_map, &mut app_state, &mut game_state);
            }
        }
    }
}

fn apply_gravity(
    tile_map: &Res<TileMap>,
    game_state: &mut ResMut<GameState>,
    player_transform: &mut Transform,
    mut player_health: &mut Health,
) {
    let next_position = player_transform.translation.clone();
    let mut tile_under_player = get_nearest_tile_on_grid(next_position.x, next_position.y);
    tile_under_player.1 -= 1;

    game_state.player_is_falling = match tile_map.0.get(&tile_under_player) {
        Some(_) => false,
        None => true,
    };

    if game_state.player_is_falling {
        player_transform.translation.y -= TILE_SIZE as f32;
        if !is_position_in_bounds(player_transform.translation.y) {
            player_health.0 = 0; // the player has fallen to their death
        }
    }
}

pub fn run_if_player_speed_doubled(player_speed_query: Query<&Speed, With<Player>>) -> ShouldRun {
    if let Ok(player_speed) = player_speed_query.get_single() {
        if (*player_speed).0 > 1 {
            ShouldRun::Yes
        } else {
            ShouldRun::No
        }
    } else {
        ShouldRun::No
    }
}

pub fn apply_speed_transparent_to_player(
    game_state: Res<GameState>,
    player_speed_query: Query<&Speed, With<Player>>,
    mut sprite_query: Query<&mut Sprite, With<Player>>,
) {
    if let Ok(mut sprite) = sprite_query.get_single_mut() {
        if let Ok(player_speed) = player_speed_query.get_single() {
            if game_state.player_num_actions_taken % player_speed.0 as u32 == 1 {
                sprite.color.set_a(0.5);
            } else {
                sprite.color.set_a(1.0);
            }
        }
    }
}

pub fn run_if_player_moved(game_state: Res<GameState>) -> ShouldRun {
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

pub fn apply_damage_to_player(
    mut commands: Commands,
    mut player_query: Query<(&mut Health, &Damage, &mut Transform), With<Player>>,
    mut obstacle_query: Query<
        (Entity, &Blocking, &mut Health, &Damage, &Transform),
        Without<Player>,
    >,
    game_state: Res<GameState>,
    game_sounds: Res<GameSounds>,
    audio: Res<Audio>,
) {
    println!("update_world");

    if let Ok((mut player_health, player_damage, mut player_transform)) =
        player_query.get_single_mut()
    {
        for (
            obstacle_entity,
            obstacle_blocking,
            mut obstacle_health,
            obstacle_damage,
            obstacle_transform,
        ) in obstacle_query.iter_mut()
        {
            if entities_are_overlapping(&player_transform, obstacle_transform) {
                let mut obstacle_just_died = false;
                if player_damage.0 > 0 && obstacle_health.0 > 0 {
                    println!("player dealt {} damage to an obstacle", player_damage.0);
                    obstacle_health.0 -= player_damage.0;
                    if obstacle_health.0 <= 0 {
                        commands.entity(obstacle_entity).despawn();
                        obstacle_just_died = true;
                    }
                }

                if !obstacle_just_died && obstacle_blocking.0 {
                    // obstacle wasn't fully killed, push back player
                    player_transform.translation = game_state.player_previous_pos;
                    if game_state.player_is_falling {
                        // compensate for gravity when falling onto the obstacle
                        player_transform.translation.y -= TILE_SIZE as f32;
                    }
                }

                if obstacle_damage.0 > 0 && player_health.0 > 0 {
                    println!("obstacle dealt {} damage to the player", obstacle_damage.0);
                    player_health.0 -= obstacle_damage.0;

                    let mut rng = rand::thread_rng();
                    audio.play(
                        game_sounds.player_hit_sfxs
                            [rng.gen_range(0..game_sounds.player_hit_sfxs.len())]
                        .clone(),
                    );
                }
            }
        }
    }
}

pub fn spawn_falling_ice_over_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut tile_map: ResMut<TileMap>,
    player_query: Query<&Transform, With<Player>>,
    game_sounds: Res<GameSounds>,
    audio: Res<Audio>,
) {
    let transform = player_query.single();
    let (x, y) = get_nearest_tile_on_grid(transform.translation.x, transform.translation.y);

    for j in y..WORLD_SIZE {
        let tile_to_inspect = (x, j);
        match tile_map.0.get(&tile_to_inspect) {
            Some(TileType::Wall) => {
                // println!("Found wall!");
                break;
            }
            Some(TileType::FallingIce) => {
                // println!("Found ice!");
                tile_map.0.remove(&tile_to_inspect);

                let sprite_pos = tile_pos_to_sprite_pos(x, j);
                commands.spawn_bundle(FallingIceBundle {
                    sprite_bundle: SpriteBundle {
                        texture: asset_server.load("FallingIce.png"),
                        transform: Transform {
                            translation: sprite_pos,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    health: Health(1),
                    damage: Damage(100),
                    falling_ice: FallingIce::default(),
                });

                audio.play(game_sounds.falling_ice_sfx.clone());

                break;
            }
            Some(TileType::Ladder) => {} // go through
            None => {}                   // keep going
        }
    }
}

pub fn move_falling_ice(
    mut commands: Commands,
    mut ice_query: Query<(Entity, &mut Transform), With<FallingIce>>,
) {
    for (entity, mut transform) in ice_query.iter_mut() {
        let t = transform.translation.clone();
        transform.translation = Vec3::new(t.x, t.y - TILE_SIZE as f32, t.z);

        let (x, y) = get_nearest_tile_on_grid(transform.translation.x, transform.translation.y);
        if x < 0 || y < 0 {
            commands.entity(entity).despawn();
        }
    }
}

const ASPECT_RATIO: f32 = 16.0 / 9.0;

pub fn fit_camera_inside_current_level(
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
