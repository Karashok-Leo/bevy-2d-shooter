use bevy::app::plugin_group;

pub mod bullet;
pub mod collision;
pub mod damage;
pub mod despawn;
pub mod enemy;
pub mod gun;
pub mod in_game;
pub mod map;
pub mod owner;
pub mod player;
pub mod water;

plugin_group! {
    pub struct WorldPlugins{
        map:::MapPlugin,
        water:::WaterPlugin,
        player:::PlayerPlugin,
        gun:::GunPlugin,
        bullet:::BulletPlugin,
        enemy:::EnemyPlugin,
        in_game:::InGamePlugin,
        damage:::DamagePlugin,
    }
}
