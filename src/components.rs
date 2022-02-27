use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use std::collections::HashMap;

pub enum TileType {
    Wall,
    Ladder,
}

#[derive(Default)]
pub struct TileMap(pub HashMap<(u32, u32), TileType>);

pub struct TurnState {
    pub player_just_took_turn: bool,
}

#[derive(Clone, Component)]
pub struct Speed(pub f32);

#[derive(Clone, Component)]
pub struct Damage(pub u16);

#[derive(Clone, Component)]
pub struct Health(pub u16);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Bundle)]
pub struct PlayerBundle {
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub player: Player,
    pub speed: Speed,
    pub damage: Damage,
    pub health: Health,
}

impl LdtkEntity for PlayerBundle {
    fn bundle_entity(
        _: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> PlayerBundle {
        PlayerBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("Player.png"),
                ..Default::default()
            },
            player: Player::default(),
            speed: Speed(1.0),
            damage: Damage(0),
            health: Health(100),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Climbable;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct LadderBundle {
    pub climbable: Climbable,
}
