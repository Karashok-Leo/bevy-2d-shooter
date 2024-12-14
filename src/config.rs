use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

#[derive(Resource, Default, Debug, Serialize, Deserialize)]
pub struct GameConfig {
    pub basic: BasicConfig,
    pub sprite: SpriteConfig,
    pub world: WorldConfig,
    pub player: PlayerConfig,
    pub enemy: EnemyConfig,
    pub gun: GunConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicConfig {
    pub window_width: f32,
    pub window_height: f32,
    pub debug: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteConfig {
    pub sprite_sheet_path: String,
    pub sprite_sheet_width: u32,
    pub sprite_sheet_height: u32,
    pub tile_w: u32,
    pub tile_h: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldConfig {
    pub num_world_decorations: usize,
    pub world_width: f32,
    pub world_height: f32,
    pub background_color: (u8, u8, u8),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerConfig {
    pub player_health: f32,
    pub player_speed: f32,
    pub player_damage_cooldown: f32,
    pub player_hurt_radius: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnemyConfig {
    pub enemy_health: f32,
    pub enemy_speed: f32,
    pub enemy_damage: f32,
    pub enemy_damage_cooldown: f32,
    pub enemy_hurt_radius: f32,
    pub max_num_enemies: usize,
    pub spawn_rate_per_second: usize,
    pub enemy_spawn_interval: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GunConfig {
    pub bullet_damage: f32,
    pub bullet_speed: f32,
    pub bullet_spawn_interval: f32,
    pub bullet_lifetime: f32,
    pub num_bullets_per_shot: usize,
}

#[derive(Default)]
pub struct ConfigPlugin;

pub fn get_config() -> GameConfig {
    match read_or_create_config() {
        Ok(config) => {
            log::info!("Config loaded successfully");
            config
        }
        Err(err) => {
            log::error!("Error loading config: {}", err);
            log::info!("Using default config");
            GameConfig::default()
        }
    }
}

fn read_or_create_config() -> Result<GameConfig, Box<dyn Error>> {
    let path = Path::new("config.toml");

    if path.exists() {
        let contents = fs::read_to_string(path)?;
        let config: GameConfig = toml::from_str(&contents)?;
        Ok(config)
    } else {
        let default_config = GameConfig::default();
        let toml_string = toml::to_string(&default_config)?;

        let mut file = File::create(path)?;
        file.write_all(toml_string.as_bytes())?;

        Ok(default_config)
    }
}

impl Default for BasicConfig {
    fn default() -> Self {
        Self {
            window_width: 1280.0,
            window_height: 720.0,
            debug: false,
        }
    }
}

impl Default for SpriteConfig {
    fn default() -> Self {
        Self {
            sprite_sheet_path: "sprites.png".to_string(),
            sprite_sheet_width: 8,
            sprite_sheet_height: 8,
            tile_w: 16,
            tile_h: 16,
        }
    }
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            num_world_decorations: 500,
            world_width: 3000.0,
            world_height: 2500.0,
            background_color: (197, 204, 184),
        }
    }
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            player_health: 100.0,
            player_speed: 80.0,
            player_damage_cooldown: 0.3,
            player_hurt_radius: 5.0,
        }
    }
}

impl Default for EnemyConfig {
    fn default() -> Self {
        Self {
            enemy_health: 100.0,
            enemy_speed: 40.0,
            enemy_damage: 20.0,
            enemy_damage_cooldown: 0.15,
            enemy_hurt_radius: 6.0,
            max_num_enemies: 20000,
            spawn_rate_per_second: 500,
            enemy_spawn_interval: 1.0,
        }
    }
}

impl Default for GunConfig {
    fn default() -> Self {
        Self {
            bullet_damage: 20.0,
            bullet_speed: 600.0,
            bullet_spawn_interval: 0.1,
            bullet_lifetime: 0.5,
            num_bullets_per_shot: 10,
        }
    }
}
