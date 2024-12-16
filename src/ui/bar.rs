use bevy::prelude::*;

#[derive(Component, Default)]
pub struct BarWidth {
    total: f32,
    ratio: f32,
}

#[derive(Component)]
pub enum BarTargetWidth {
    Background(f32),
    Foreground(f32),
}

#[derive(Default)]
pub struct BarPlugin;

impl BarWidth {
    const GRADUAL_CHANGE_SPEED: f32 = 0.005;

    pub fn new(total: f32) -> Self {
        Self { total, ratio: 1.0 }
    }

    #[allow(dead_code)]
    pub fn get_total_width(&self) -> f32 {
        self.total
    }

    pub fn change_suddenly(&mut self, target: f32) {
        self.ratio = target;
    }

    pub fn change_gradually(&mut self, target: f32) {
        self.ratio = self.ratio.lerp(target, Self::GRADUAL_CHANGE_SPEED);
    }
}

impl BarTargetWidth {
    pub fn set_target(&mut self, target: f32) {
        match self {
            Self::Background(value) => *value = target,
            Self::Foreground(value) => *value = target,
        }
    }

    pub fn unwrap(&self) -> f32 {
        match self {
            Self::Background(value) => *value,
            Self::Foreground(value) => *value,
        }
    }
}

impl Plugin for BarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_bar_width, update_bar_node));
    }
}

fn update_bar_node(mut query: Query<(&mut Node, &BarWidth)>) {
    for (mut node, width) in query.iter_mut() {
        node.width = Val::Px(width.total * width.ratio);
    }
}

fn update_bar_width(mut query: Query<(&mut BarWidth, &BarTargetWidth)>) {
    for (mut width, target_width) in query.iter_mut() {
        match target_width {
            BarTargetWidth::Background(target) => {
                // decrease - background changes gradually
                if *target < width.ratio {
                    width.change_gradually(*target);
                }
                // increase - background changes suddenly
                else {
                    width.change_suddenly(*target);
                }
            }
            BarTargetWidth::Foreground(target) => {
                // increase - foreground changes gradually
                if *target > width.ratio {
                    width.change_gradually(*target);
                }
                // decrease - foreground changes suddenly
                else {
                    width.change_suddenly(*target);
                }
            }
        }
    }
}
