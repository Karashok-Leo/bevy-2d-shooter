pub const DEBUG: bool = false;

// Window
pub const WW: f32 = 1280.0;
pub const WH: f32 = 720.0;

// Sprites
pub const SPRITE_SHEET_PATH: &str = "sprites.png";
pub const CAMERA_SCALING: f32 = 0.32;
pub const TILE_W: u32 = 16;
pub const TILE_H: u32 = 16;
pub const SPRITE_SHEET_W: u32 = 8;
pub const SPRITE_SHEET_H: u32 = 8;

pub enum SpriteOrder {
    GRASS,
    Player,
    Gun,
    Bullet,
    Enemy,
}

impl SpriteOrder {
    pub fn z_index(&self) -> f32 {
        match self {
            SpriteOrder::GRASS => 0.0,
            SpriteOrder::Player => 2.0,
            SpriteOrder::Gun => 0.1,
            SpriteOrder::Bullet => 1.0,
            SpriteOrder::Enemy => 1.0,
        }
    }
}

// World
pub const NUM_WORLD_DECORATIONS: usize = 500;
pub const WORLD_W: f32 = 3000.0;
pub const WORLD_H: f32 = 2500.0;

// Player
pub const PLAYER_SPEED: f32 = 80.0;
pub const PLAYER_HEALTH: f32 = 100.0;
pub const PLAYER_DAMAGE_COOLDOWN: f32 = 0.3;
pub const PLAYER_HURT_RADIUS: f32 = 5.0;

// Enemy
pub const MAX_NUM_ENEMIES: usize = 20000;
pub const ENEMY_DAMAGE: f32 = 20.0;
pub const ENEMY_DAMAGE_COOLDOWN: f32 = 0.15;
pub const SPAWN_RATE_PER_SECOND: usize = 500;
pub const ENEMY_HEALTH: f32 = 100.0;
pub const ENEMY_SPAWN_INTERVAL: f32 = 1.0;
pub const ENEMY_SPEED: f32 = 40.0;
pub const ENEMY_HURT_RADIUS: f32 = 6.0;

// Kd-tree
pub const KD_TREE_REFRESH_RATE: f32 = 0.1;

// Gun
pub const BULLET_SPAWN_INTERVAL: f32 = 0.1;
pub const BULLET_TIME_SECS: f32 = 0.5;
pub const BULLET_SPEED: f32 = 600.0;
pub const BULLET_DAMAGE: f32 = 20.0;

pub const NUM_BULLETS_PER_SHOT: usize = 10;

// Colors
pub const BG_COLOR: (u8, u8, u8) = (197, 204, 184);
