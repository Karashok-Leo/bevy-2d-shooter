use crate::resource::GlobalFont;
use crate::ui::hud::Hud;
use crate::ui::popup::popup_text;
use crate::world::damage::DamageEvent;
use crate::world::enemy::Enemy;
use bevy::prelude::*;

pub fn on_enemy_damaged(
    mut commands: Commands,
    font: Res<GlobalFont>,
    mut event_reader: EventReader<DamageEvent>,
    mut enemy_query: Query<&GlobalTransform, With<Enemy>>,
) {
    for event in event_reader.read() {
        if !event.apply {
            continue;
        }
        if let Ok(transform) = enemy_query.get_mut(event.target) {
            // pop text
            let mut pop_transform = transform.compute_transform();
            pop_transform.translation.z += 100.0;
            commands.spawn((
                Hud,
                popup_text(
                    font.handle.clone(),
                    event.context.damage.to_string(),
                    pop_transform,
                    TextColor(Color::srgb(0.8, 0.1, 0.1)),
                ),
            ));
        }
    }
}
