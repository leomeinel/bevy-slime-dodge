/*
 * File: animation.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! Player sprite animation.
//! This is based on multiple examples and may be very different for your game.
//! - [Sprite flipping](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_flipping.rs)
//! - [Sprite animation](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_animation.rs)
//! - [Timers](https://github.com/bevyengine/bevy/blob/latest/examples/time/timers.rs)

pub(crate) mod npc;
pub(crate) mod player;

use bevy::{ecs::component::Mutable, prelude::*};
use bevy_prng::WyRand;
use bevy_rand::{global::GlobalRng, traits::ForkableSeed as _};
use bevy_rapier2d::prelude::*;
use rand::seq::IndexedRandom as _;
use std::time::Duration;

use crate::{audio::sound_effect, characters::CharacterAssets};

pub(super) fn plugin(app: &mut App) {
    // Setup rng source
    app.add_systems(Startup, setup_rng);

    app.add_plugins((npc::plugin, player::plugin));
}

#[derive(Reflect, PartialEq, Clone, Copy)]
pub enum MovementAnimationState {
    Idling,
    Walking,
}

#[derive(Component)]
pub(crate) struct Rng;

fn setup_rng(mut commands: Commands, mut global: Single<&mut WyRand, With<GlobalRng>>) {
    commands.spawn((Rng, global.fork_seed()));
}

pub(crate) trait MovementAnimation {
    /// The number of idle frames.
    const IDLE_FRAMES: usize = 1;
    /// The duration of each idle frame.
    const IDLE_INTERVAL: Duration = Duration::from_millis(500);
    /// The number of walking frames.
    const WALKING_FRAMES: usize = 8;
    /// The duration of each walking frame.
    const WALKING_INTERVAL: Duration = Duration::from_millis(100);

    fn idling() -> Self;

    fn walking() -> Self;

    fn new() -> Self;

    /// Update animation timers.
    fn update_timer(&mut self, delta: Duration);

    /// Update animation state if it changes.
    fn update_state(&mut self, state: MovementAnimationState);

    /// Whether animation changed this tick.
    fn changed(&self) -> bool;

    /// Return sprite index in the atlas.
    fn get_atlas_index(&self) -> usize;

    fn get_frame(&self) -> usize;
    fn get_state(&self) -> MovementAnimationState;
}

pub(crate) trait SoundFrames {
    fn get_frames(&self) -> &Vec<usize>;
}

/// Update the animation timer.
pub(crate) fn update_animation_timer<T: Component<Mutability = Mutable> + MovementAnimation>(
    time: Res<Time>,
    mut query: Query<&mut T>,
) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

/// Update the sprite direction and animation state (idling/walking).
pub(crate) fn update_animation_movement<T: Component<Mutability = Mutable> + MovementAnimation>(
    mut query: Query<(&KinematicCharacterController, &mut Sprite, &mut T)>,
) {
    for (controller, mut sprite, mut animation) in &mut query {
        let Some(intent) = controller.translation else {
            return;
        };
        let dx = intent.x;
        if dx != 0. {
            sprite.flip_x = dx < 0.;
        }

        let animation_state = if intent == Vec2::ZERO {
            MovementAnimationState::Idling
        } else {
            MovementAnimationState::Walking
        };
        animation.update_state(animation_state);
    }
}

/// Update the texture atlas to reflect changes in the animation.
pub(crate) fn update_animation_atlas<T: Component<Mutability = Mutable> + MovementAnimation>(
    mut query: Query<(&T, &mut Sprite)>,
) {
    for (animation, mut sprite) in &mut query {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };
        if animation.changed() {
            atlas.index = animation.get_atlas_index();
        }
    }
}

/// If the character is moving, play a step sound effect synchronized with the
/// animation.
pub(crate) fn trigger_step_sound_effect<
    T: Component<Mutability = Mutable> + MovementAnimation,
    A: Resource + CharacterAssets<Animation = T>,
    B: Resource + SoundFrames,
>(
    mut commands: Commands,
    assets: If<Res<A>>,
    animation_frames: Res<B>,
    mut step_query: Query<&T>,
    mut rng_query: Single<&mut WyRand, With<Rng>>,
) {
    for animation in &mut step_query {
        if animation.get_state() == MovementAnimationState::Walking
            && animation.changed()
            && animation_frames
                .get_frames()
                .contains(&animation.get_frame())
        {
            let random_step = assets
                .get_step_sounds()
                .choose(rng_query.as_mut())
                .unwrap()
                .clone();
            commands.spawn(sound_effect(random_step));
        }
    }
}
