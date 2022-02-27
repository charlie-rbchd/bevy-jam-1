use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use std::collections::HashMap;
use std::string::String;

pub enum TileType {
    Wall,
    Ladder,
    FallingIce
}

#[derive(Default)]
pub struct TileMap(pub HashMap<(i32, i32), TileType>);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
}

pub struct TurnState {
    pub player_just_took_turn: bool,
    pub player_is_falling: bool,
}

#[derive(Clone, Component)]
pub struct Speed(pub f32);

#[derive(Clone, Component)]
pub struct Damage(pub i32);

#[derive(Clone, Component)]
pub struct Health(pub i32);

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
    ) -> Self {
        // TODO: different starting stats depending on which advantage is chosen

        Self {
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
pub struct Obstacle;

#[derive(Clone, Bundle)]
pub struct ObstacleBundle {
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub obstacle: Obstacle,
    pub speed: Speed,
    pub damage: Damage,
    pub health: Health,
}

impl LdtkEntity for ObstacleBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> Self {
        let mut speed = Speed(1.0);
        if let Some(speed_field) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == "Speed".to_string())
        {
            if let FieldValue::Float(Some(speed_value)) = speed_field.value {
                speed = Speed(speed_value);
            }
        }

        let mut damage = Damage(0);
        if let Some(damage_field) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == "Damage".to_string())
        {
            if let FieldValue::Int(Some(damage_value)) = damage_field.value {
                damage = Damage(damage_value);
            }
        }

        let mut health = Health(100);
        if let Some(health_field) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == "Health".to_string())
        {
            if let FieldValue::Int(Some(health_value)) = health_field.value {
                health = Health(health_value);
            }
        }

        let mut texture_filename = String::new();
        if let Some(type_field) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == "Type".to_string())
        {
            if let FieldValue::Enum(Some(type_value)) = &type_field.value {
                texture_filename = format!("{}.png", type_value);
            }
        }

        Self {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load(&texture_filename),
                ..Default::default()
            },
            obstacle: Obstacle::default(),
            speed,
            damage,
            health,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct WallTile;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallTileBundle {
    wall: WallTile,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct ClimbableTile;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ClimableTileBundle {
    pub climbable: ClimbableTile,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct FallingIceTile;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct FallingIceTileBundle {
    pub falling_ice: FallingIceTile,
}

// The actual falling ice, when the player goes underneath
// the falling ice tile
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct FallingIce;

#[derive(Clone, Bundle)]
pub struct FallingIceBundle {
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub falling_ice: FallingIce,
}

impl LdtkEntity for FallingIceBundle {
    fn bundle_entity(
        _: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> Self {
        let texture_filename = String::from("FallingIce_Dynamic.png");
        Self {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load(&texture_filename),
                ..Default::default()
            },
            falling_ice: FallingIce::default(),
        }
    }
}
