use std::{marker::PhantomData, time::Duration};

use bevy::prelude::*;

use crate::{animations::prelude::*, log::prelude::*, render::prelude::*};

/// Jump duration in milliseconds for `T`.
#[derive(Resource, Debug, Default)]
pub(crate) struct JumpDuration<T>
where
    T: Visible,
{
    pub(crate) millis: u64,
    _phantom: PhantomData<T>,
}
impl<T> JumpDuration<T>
where
    T: Visible,
{
    pub(crate) fn from_millis(millis: u64) -> Self {
        Self {
            millis,
            ..default()
        }
    }
}

/// Timer that tracks jumping.
#[derive(Component, Default, Debug, Clone, PartialEq, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub(crate) struct JumpTimer(pub(crate) Timer);
impl JumpTimer {
    pub(crate) fn from_millis(millis: u64) -> Self {
        Self(Timer::new(Duration::from_millis(millis), TimerMode::Once))
    }
}

/// [`Character`](crate::characters::Character) jump height.
#[derive(Component, Default)]
pub(crate) struct JumpHeight {
    pub(crate) max: f32,
    pub(crate) current: f32,
}
impl JumpHeight {
    pub(crate) fn new(max: f32) -> Self {
        Self { max, ..default() }
    }
}

/// Insert [`JumpTimer`] on [`AnimationAction::Jump`].
pub(super) fn insert_timer<T>(
    container_query: Query<(Entity, &AnimationState), (With<T>, Without<JumpTimer>)>,
    mut commands: Commands,
    jump_duration: Res<JumpDuration<T>>,
) where
    T: Visible,
{
    for (entity, state) in container_query {
        if state.action == AnimationAction::Jump {
            // NOTE: Using try here is necessary since the entity might have been despawned elsewhere.
            commands
                .entity(entity)
                .try_insert(JumpTimer::from_millis(jump_duration.millis));
        }
    }
}

/// Move sprite according to progress of [`JumpTimer`].
pub(super) fn move_sprite<T>(
    container_query: Query<(&mut JumpHeight, &JumpTimer, &Children), With<T>>,
    mut base_query: Query<&mut Transform, With<AnimationBase>>,
) where
    T: Visible,
{
    for (mut height, timer, children) in container_query {
        let factor = EaseFunction::QuadraticOut
            .ping_pong()
            .expect(ERR_INVALID_DOMAIN_EASING);
        // NOTE: We are multiplying by 2 since `PingPongCurve` has a domain from 0 to 2.
        let factor = factor.sample_clamped(timer.fraction() * 2.);
        let target = height.max * factor;

        let child = children
            .iter()
            .find(|e| base_query.contains(*e))
            .expect(ERR_INVALID_CHILDREN);
        let mut transform = base_query.get_mut(child).expect(ERR_INVALID_CHILDREN);
        transform.translation.y += target - height.current;
        height.current = target;
    }
}

/// Maximum jump height.
const SHADOW_MAX_CAST_HEIGHT: f32 = 64.;

/// Scale [`MeshShadow`] with [`JumpHeight`].
pub(super) fn scale_shadow<T>(
    container_query: Query<(&JumpHeight, &Children), With<T>>,
    mut shadow_query: Query<&mut Transform, With<MeshShadow>>,
) where
    T: Visible,
{
    for (height, children) in container_query {
        let scale = 1. - height.current.clamp(0., SHADOW_MAX_CAST_HEIGHT) / SHADOW_MAX_CAST_HEIGHT;
        let child = children
            .iter()
            .find(|e| shadow_query.contains(*e))
            .expect(ERR_INVALID_CHILDREN);
        let mut transform = shadow_query.get_mut(child).expect(ERR_INVALID_CHILDREN);
        transform.scale = Vec2::splat(scale).extend(1.);
    }
}

// NOTE: This might insert undesirable idle frames but is much simpler than more sophisticated solutions. Visually these frames are not noticeable.
/// Reset jump if [`JumpTimer`] just finished.
pub(super) fn reset_jump<T>(container_query: Query<(&mut AnimationState, &JumpTimer), With<T>>)
where
    T: Visible,
{
    for (mut state, timer) in container_query {
        if timer.just_finished() {
            state.action = AnimationAction::Idle;
        }
    }
}
