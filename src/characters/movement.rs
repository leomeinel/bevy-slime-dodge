use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{animations::prelude::*, characters::prelude::*};

/// Can apply to anything that walks.
pub(crate) trait Walker
where
    Self: Character,
{
    /// [`AnimationAction`] for walking.
    ///
    /// This allows certain [`Character`]s to for example also jump while walking.
    fn walk_action() -> AnimationAction {
        AnimationAction::Walk
    }
}

/// Direction the [`Character`] is facing.
#[derive(Component)]
pub(crate) struct FacingDirection(pub(crate) Vec2);
impl Default for FacingDirection {
    fn default() -> Self {
        Self(Vec2::NEG_Y)
    }
}

/// [`Character`] walking speed.
#[derive(Component)]
pub(crate) struct WalkSpeed(pub(crate) f32);

/// Update [`FacingDirection`].
pub(super) fn update_facing_direction(
    query: Query<
        (
            &mut FacingDirection,
            Option<&AttackTimer>,
            &AimDirection,
            Option<&KinematicCharacterControllerOutput>,
        ),
        Or<(
            Changed<AimDirection>,
            Changed<KinematicCharacterControllerOutput>,
        )>,
    >,
) {
    for (mut facing, timer, aim_direction, controller_output) in query {
        let direction = if let Some(timer) = timer
            && !timer.just_finished()
            && aim_direction.0 != Vec2::ZERO
        {
            aim_direction.0
        } else if let Some(controller_output) = controller_output
            && controller_output.desired_translation != Vec2::ZERO
        {
            controller_output.desired_translation
        } else {
            return;
        }
        .normalize_or_zero();

        facing.0 = direction;
    }
}
