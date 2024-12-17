use crate::state::GameState;
use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Sprite)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut, Default)]
#[require(AnimationIndices, Sprite)]
pub struct AnimationTimer(pub Timer);

pub struct AnimatorPlugin;

impl AnimationIndices {
    pub fn new(first: usize, last: usize) -> Self {
        Self { first, last }
    }

    pub fn from_length(first: usize, length: usize) -> Self {
        Self {
            first,
            last: first + length - 1,
        }
    }

    #[inline]
    pub fn with_first(&mut self, first: usize) {
        let diff = self.last - self.first;
        self.first = first;
        self.last = first + diff;
    }

    #[inline]
    pub fn with_length(&mut self, length: usize) {
        self.last = self.first + length - 1;
    }
}

impl Plugin for AnimatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate.run_if(in_state(GameState::Running)));
    }
}

fn animate(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index < indices.first || atlas.index >= indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                }
            }
        }
    }
}
