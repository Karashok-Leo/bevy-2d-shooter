use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

#[derive(Resource, Default, Debug, Serialize, Deserialize)]
pub struct GameConfig {
    pub water: WaterConfig,
    pub basic: BasicConfig,
    pub world: WorldConfig,
    pub map: MapConfig,
    pub player: PlayerConfig,
    pub enemy: EnemyConfig,
    pub bullet: BulletConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterConfig {
    pub radial_scale: f32,
    pub axial_scale: f32,
    pub contrast: f32,
    pub speed: f32,
    pub intensity: f32,
    pub color_offset: Vec3,
}

impl Default for WaterConfig {
    fn default() -> Self {
        Self {
            radial_scale: 10.0,
            axial_scale: 4.0,
            contrast: 8.0,
            speed: 0.5,
            intensity: 0.1,
            color_offset: Vec3::new(-0.1, 0.3, 0.9),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicConfig {
    pub tile_size: f32,
    pub window_width: f32,
    pub window_height: f32,
    pub debug: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldConfig {
    pub num_world_decorations: usize,
    pub world_width: f32,
    pub world_height: f32,
    pub background_color: (u8, u8, u8),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapConfig {
    pub map_w: u32,
    pub map_h: u32,
    pub scale: f32,
    pub grass_height: f32,
    pub sand_height: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerConfig {
    pub health: f32,
    pub speed: f32,
    pub damage_cooldown: f32,
    pub collider_size: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnemyConfig {
    pub health: f32,
    pub speed: f32,
    pub damage: f32,
    pub damage_cooldown: f32,
    pub follow_range: f32,
    pub collider_size: f32,
    pub spawn_dummy: bool,
    pub spawn_waves: bool,
    pub spawn_limit: usize,
    pub spawn_rate_per_second: usize,
    pub spawn_interval: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulletConfig {
    pub damage: f32,
    pub speed: f32,
    pub spawn_interval: f32,
    pub lifetime: f32,
    pub num_per_shot: usize,
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
        log::info!("Config loaded from file");
        Ok(config)
    } else {
        log::info!("Config file not found, creating default config");
        let default_config = GameConfig::default();
        let toml_string = toml::to_string(&default_config)?;

        let mut file = File::create(path)?;
        file.write_all(toml_string.as_bytes())?;

        Ok(default_config)
    }
}

fn reload_config(mut commands: Commands) {
    commands.insert_resource(get_config());
}

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            reload_config.run_if(input_just_pressed(KeyCode::KeyR)),
        );
    }
}

impl Default for BasicConfig {
    fn default() -> Self {
        Self {
            tile_size: 16.0,
            window_width: 1280.0,
            window_height: 720.0,
            debug: false,
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

impl Default for MapConfig {
    fn default() -> Self {
        Self {
            map_w: 128,
            map_h: 96,
            scale: 30.0,
            grass_height: 1.0,
            sand_height: 0.2,
        }
    }
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            health: 100.0,
            speed: 80.0,
            damage_cooldown: 0.3,
            collider_size: 5.0,
        }
    }
}

impl Default for EnemyConfig {
    fn default() -> Self {
        Self {
            health: 100.0,
            speed: 40.0,
            damage: 20.0,
            damage_cooldown: 0.15,
            follow_range: 1000.0,
            collider_size: 6.0,
            spawn_dummy: false,
            spawn_waves: true,
            spawn_limit: 20000,
            spawn_rate_per_second: 500,
            spawn_interval: 1.0,
        }
    }
}

impl Default for BulletConfig {
    fn default() -> Self {
        Self {
            damage: 20.0,
            speed: 600.0,
            spawn_interval: 0.1,
            lifetime: 0.5,
            num_per_shot: 10,
        }
    }
}
