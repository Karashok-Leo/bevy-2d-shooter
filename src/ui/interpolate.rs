use bevy::prelude::*;
use bevy_tween::interpolate::Interpolator;
use bevy_tween::prelude::ComponentTween;
use bevy_tween::{component_tween_system, BevyTweenRegisterSystems, DefaultTweenPlugins};

/// [`Interpolator`] for Bevy's [`TextColor`](bevy::prelude::TextColor) used in UIs.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct TextColor {
    #[allow(missing_docs)]
    pub start: Color,
    #[allow(missing_docs)]
    pub end: Color,
}

/// Constructor for [`TextColor`](crate::interpolate::TextColor)
pub fn text_color(start: Color, end: Color) -> TextColor {
    TextColor { start, end }
}

/// Constructor for [`TextColor`](crate::interpolate::TextColor) that's relative to previous value using currying.
pub fn text_color_to(to: Color) -> impl Fn(&mut Color) -> TextColor {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        text_color(start, end)
    }
}

#[derive(Default)]
pub struct InterpolatePlugin;

impl Plugin for InterpolatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(DefaultTweenPlugins)
            .add_tween_systems(component_tween_system::<TextColor>())
            .register_type::<ComponentTween<bevy::prelude::TextColor>>();
    }
}

impl Interpolator for TextColor {
    type Item = bevy::prelude::TextColor;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.0 = self.start.mix(&self.end, value)
    }
}
