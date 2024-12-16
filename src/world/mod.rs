use bevy::app::plugin_group;

pub mod bullet;
pub mod collision;
pub mod damage;
pub mod enemy;
pub mod gun;
pub mod in_game;
pub mod owner;
pub mod player;

plugin_group! {
    pub struct WorldPlugins{
        player:::PlayerPlugin,
        gun:::GunPlugin,
        bullet:::BulletPlugin,
        enemy:::EnemyPlugin,
        in_game:::InGamePlugin,
        damage:::DamagePlugin,
    }
}
