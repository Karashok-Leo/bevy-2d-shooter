use bevy::ecs::component::{ComponentHooks, StorageType};
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;

pub fn physical_transform(transform: Transform) -> impl Bundle {
    (
        transform,
        Velocity::default(),
        PhysicalTranslation(transform.translation),
        PreviousPhysicalTranslation(transform.translation),
    )
}

/// A vector representing the player's velocity in the physics simulation.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
#[require(PhysicalTranslation, PreviousPhysicalTranslation)]
pub struct Velocity(pub Vec3);

/// The actual position of the player in the physics simulation.
/// This is separate from the `Transform`, which is merely a visual representation.
///
/// If you want to make sure that this component is always initialized
/// with the same value as the `Transform`'s translation, you can
/// use a [component lifecycle hook](https://docs.rs/bevy/0.14.0/bevy/ecs/component/struct.ComponentHooks.html)
#[derive(Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PhysicalTranslation(pub Vec3);

/// The value [`PhysicalTranslation`] had in the last fixed timestep.
/// Used for interpolation in the `interpolate_rendered_transform` system.
#[derive(Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PreviousPhysicalTranslation(pub Vec3);

pub struct PhysicsPlugin;

impl Component for PhysicalTranslation {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    // fn register_component_hooks(_hooks: &mut ComponentHooks) {
    //     _hooks.on_add(|mut world: DeferredWorld, entity, _component_id| {
    //         let transform_translation = world.get::<Transform>(entity).unwrap().translation;
    //         let mut physical_translation = world.get_mut::<PhysicalTranslation>(entity).unwrap();
    //         physical_translation.0 = transform_translation;
    //     });
    // }
}

impl Component for PreviousPhysicalTranslation {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    // fn register_component_hooks(_hooks: &mut ComponentHooks) {
    //     _hooks.on_add(|mut world: DeferredWorld, entity, _component_id| {
    //         let physical_translation = world.get::<PhysicalTranslation>(entity).unwrap().0;
    //         let mut previous_physical_translation = world
    //             .get_mut::<PreviousPhysicalTranslation>(entity)
    //             .unwrap();
    //         previous_physical_translation.0 = physical_translation;
    //     });
    // }
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, advance_physics).add_systems(
            // The `RunFixedMainLoop` schedule allows us to schedule systems to run before and after the fixed timestep loop.
            RunFixedMainLoop,
            (
                // The player's visual representation needs to be updated after the physics simulation has been advanced.
                // This could be run in `Update`, but if we run it here instead, the systems in `Update`
                // will be working with the `Transform` that will actually be shown on screen.
                interpolate_rendered_transform.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
            ),
        );
    }
}

/// Advance the physics simulation by one fixed timestep. This may run zero or multiple times per frame.
///
/// Note that since this runs in `FixedUpdate`, `Res<Time>` would be `Res<Time<Fixed>>` automatically.
/// We are being explicit here for clarity.
fn advance_physics(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut PhysicalTranslation,
        &mut PreviousPhysicalTranslation,
        &mut Velocity,
    )>,
) {
    for (mut current_physical_translation, mut previous_physical_translation, mut velocity) in
        query.iter_mut()
    {
        previous_physical_translation.0 = current_physical_translation.0;
        current_physical_translation.0 += velocity.0 * fixed_time.delta_secs();
        velocity.0 = Vec3::ZERO;
    }
}

fn interpolate_rendered_transform(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut Transform,
        &PhysicalTranslation,
        &PreviousPhysicalTranslation,
    )>,
) {
    for (mut transform, current_physical_translation, previous_physical_translation) in
        query.iter_mut()
    {
        let previous = previous_physical_translation.0;
        let current = current_physical_translation.0;
        // The overstep fraction is a value between 0 and 1 that tells us how far we are between two fixed timesteps.
        let alpha = fixed_time.overstep_fraction();

        let rendered_translation = previous.lerp(current, alpha);
        transform.translation = rendered_translation;
    }
}
