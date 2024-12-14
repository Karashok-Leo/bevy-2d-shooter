use crate::resource::GlobalTextureAtlas;
use crate::ui::bar::*;
use crate::world::damage::Health;
use crate::world::player::Player;
use bevy::prelude::*;
use bevy::ui::widget::NodeImageMode;

const PLAYER_HEALTH_BAR_WIDTH: f32 = 400.0;

#[derive(Component)]
pub struct PlayerHealthBar;

pub fn update_player_health_bar(
    player_health: Single<&Health, With<Player>>,
    mut bar_query: Query<&mut BarTargetWidth, With<PlayerHealthBar>>,
) {
    for mut bar_target in bar_query.iter_mut() {
        bar_target.set_target(player_health.current() / player_health.max());
    }
}

pub fn spawn_player_health_bar(mut commands: Commands, texture_atlas: Res<GlobalTextureAtlas>) {
    let slicer = TextureSlicer {
        border: BorderRect::square(2.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 10.0,
    };
    commands
        .spawn((
            crate::ui::hud::Hud,
            Visibility::Hidden,
            Node {
                right: Val::Px(10.0),
                top: Val::Px(10.0),
                position_type: PositionType::Absolute,
                ..default()
            },
        ))
        .with_children(|parent| {
            let bar_node = Node {
                width: Val::Px(PLAYER_HEALTH_BAR_WIDTH),
                height: Val::Px(40.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                ..default()
            };
            let outer_bar_atlas = TextureAtlas {
                layout: texture_atlas.layout.clone().unwrap(),
                index: 62,
            };
            parent
                .spawn((
                    bar_node.clone(),
                    ImageNode::from_atlas_image(
                        texture_atlas.image.clone().unwrap(),
                        outer_bar_atlas,
                    )
                    .with_mode(NodeImageMode::Sliced(slicer.clone())),
                ))
                .with_children(|parent| {
                    let inner_bar_atlas = TextureAtlas {
                        layout: texture_atlas.layout.clone().unwrap(),
                        index: 63,
                    };
                    parent
                        .spawn((
                            bar_node.clone(),
                            ImageNode::from_atlas_image(
                                texture_atlas.image.clone().unwrap(),
                                inner_bar_atlas.clone(),
                            )
                            .with_color(Color::srgb(0.8, 0.2, 0.2).with_alpha(0.6))
                            .with_mode(NodeImageMode::Sliced(slicer.clone())),
                            BarWidth::new(PLAYER_HEALTH_BAR_WIDTH),
                            BarTargetWidth::Background(1.0),
                            PlayerHealthBar,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                bar_node,
                                ImageNode::from_atlas_image(
                                    texture_atlas.image.clone().unwrap(),
                                    inner_bar_atlas,
                                )
                                .with_mode(NodeImageMode::Sliced(slicer.clone())),
                                BarWidth::new(PLAYER_HEALTH_BAR_WIDTH),
                                BarTargetWidth::Foreground(1.0),
                                PlayerHealthBar,
                            ));
                        });
                });
        });
}
