use crate::config::GameConfig;
use crate::state::GameState::GameInit;
use crate::world::in_game::InGameScoped;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, Material2dPlugin};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct Water {
    #[uniform(0)]
    radial_scale: f32,
    #[uniform(1)]
    axial_scale: f32,
    #[uniform(2)]
    contrast: f32,
    #[uniform(3)]
    speed: f32,
    #[uniform(4)]
    intensity: f32,
    #[uniform(5)]
    color_offset: Vec3,
}

#[derive(Default)]
pub struct WaterPlugin;

impl Material2d for Water {
    fn fragment_shader() -> ShaderRef {
        "water_shader.wgsl".into()
    }
}

impl Plugin for WaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((Material2dPlugin::<Water>::default(),))
            .add_systems(OnEnter(GameInit), spawn_water);
    }
}

fn spawn_water(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut water_materials: ResMut<Assets<Water>>,
    config: Res<GameConfig>,
) {
    let water = Water {
        radial_scale: config.water.radial_scale,
        axial_scale: config.water.axial_scale,
        contrast: config.water.contrast,
        speed: config.water.speed,
        intensity: config.water.intensity,
        color_offset: config.water.color_offset,
    };
    commands.spawn((
        InGameScoped,
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(water_materials.add(water)),
        Transform::from_xyz(0., 0., -100.).with_scale(Vec3::new(6000., 6000., 1.)),
    ));
}
