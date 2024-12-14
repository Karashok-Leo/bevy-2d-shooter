use bevy::ecs::schedule::*;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Health {
    max: f32,
    current: f32,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DamagePhase {
    Post,
    Before,
    Apply,
    After,
}

pub enum DamageType {
    Bullet,
    Enemy,
}

#[derive(Clone, Copy)]
pub struct DamageContext {
    pub damage: f32,
    pub damage_type: u32,
    pub attacker: Option<Entity>,
}

#[derive(Event)]
pub struct DamageEvent {
    pub target: Entity,
    pub context: DamageContext,
    pub apply: bool,
}

#[derive(Component, Default)]
pub struct DamageCooldown(Timer);

#[derive(Component)]
#[require(DamageCooldown, Sprite)]
pub struct DamageFlash;

pub struct DamagePlugin;

impl Health {
    pub fn new(max: f32) -> Self {
        Self { max, current: max }
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }

    pub fn max(&self) -> f32 {
        self.max
    }

    pub fn current(&self) -> f32 {
        self.current
    }

    fn damage(&mut self, damage: f32) {
        self.current -= damage;
        if self.current < 0.0 {
            self.current = 0.0;
        }
    }

    pub fn heal(&mut self, amount: f32) {
        self.current += amount;
        if self.current > self.max {
            self.current = self.max;
        }
    }
}

impl Into<u32> for DamageType {
    fn into(self) -> u32 {
        match self {
            DamageType::Bullet => 0,
            DamageType::Enemy => 1,
        }
    }
}

impl DamageContext {
    pub fn new(damage: f32, damage_type: impl Into<u32>, attacker: Option<Entity>) -> Self {
        Self {
            damage,
            damage_type: damage_type.into(),
            attacker,
        }
    }
    pub fn without_attacker(damage: f32, damage_type: impl Into<u32>) -> Self {
        Self::new(damage, damage_type, None)
    }
}

impl DamageCooldown {
    pub fn new(cooldown: Duration) -> Self {
        Self(Timer::new(cooldown, TimerMode::Once))
    }
}

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>()
            .configure_sets(
                Update,
                (
                    DamagePhase::Post,
                    DamagePhase::Before,
                    DamagePhase::Apply,
                    DamagePhase::After,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    apply_damage.in_set(DamagePhase::Apply),
                    update_cooldown.in_set(DamagePhase::Post),
                ),
            );
    }
}

fn apply_damage(
    mut event_mutator: EventMutator<DamageEvent>,
    mut health_query: Query<&mut Health>,
    mut cooldown_query: Query<&mut DamageCooldown>,
    mut flash_query: Query<&mut Sprite, With<DamageFlash>>,
) {
    for event in event_mutator.read() {
        // skip if the event has been cancelled
        if !event.apply {
            continue;
        }

        // skip if the target has no health or is already dead
        let Ok(mut health) = health_query.get_mut(event.target) else {
            event.apply = false;
            continue;
        };
        if !health.is_alive() {
            event.apply = false;
            continue;
        }

        let cooldown = cooldown_query.get_mut(event.target);

        if cooldown.is_ok() {
            let mut cd = cooldown.unwrap();

            // skip if the target is still on cooldown
            if !cd.0.finished() {
                event.apply = false;
                continue;
            }

            // reset the cooldown
            cd.0.reset();

            // add a damage flash effect
            if let Ok(mut sprite) = flash_query.get_mut(event.target) {
                sprite.color = Color::srgb(1.0, 0.0, 0.0);
            }
        }

        // apply the damage
        health.damage(event.context.damage);
    }
}

fn update_cooldown(
    time: Res<Time>,
    mut cooldown_query: Query<(Entity, &mut DamageCooldown)>,
    mut flash_query: Query<&mut Sprite, With<DamageFlash>>,
) {
    for (entity, mut cooldown) in cooldown_query.iter_mut() {
        cooldown.0.tick(time.delta());

        // remove the damage flash effect if the cooldown is over
        if cooldown.0.just_finished() {
            if let Ok(mut sprite) = flash_query.get_mut(entity) {
                sprite.color = Color::default();
            }
        }
    }
}
