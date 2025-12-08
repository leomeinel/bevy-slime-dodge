/*
 * File: player.rs
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

use bevy::prelude::*;
use std::time::Duration;

use crate::{
    AppSystems, PausableSystems,
    characters::{
        animation::{
            MovementAnimation, MovementAnimationState, SoundFrames, trigger_step_sound_effect,
            update_animation_atlas, update_animation_movement, update_animation_timer,
        },
        player::PlayerAssets,
    },
};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(PlayerSoundFrames(vec![5, 9]));
    // Animate and play sound effects based on controls.
    app.add_systems(
        Update,
        (
            update_animation_timer::<PlayerAnimation>.in_set(AppSystems::TickTimers),
            (
                update_animation_movement::<PlayerAnimation>,
                update_animation_atlas::<PlayerAnimation>,
                trigger_step_sound_effect::<PlayerAnimation, PlayerAssets, PlayerSoundFrames>,
            )
                .chain()
                .in_set(AppSystems::Update),
        )
            .in_set(PausableSystems),
    );
}

/// Component that tracks player's animation state.
/// It is tightly bound to the texture atlas we use.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerAnimation {
    timer: Timer,
    frame: usize,
    state: MovementAnimationState,
}

impl MovementAnimation for PlayerAnimation {
    /// The number of idle frames.
    const IDLE_FRAMES: usize = 1;
    /// The duration of each idle frame.
    const IDLE_INTERVAL: Duration = Duration::from_millis(500);
    /// The number of walking frames.
    const WALKING_FRAMES: usize = 8;
    /// The duration of each walking frame.
    const WALKING_INTERVAL: Duration = Duration::from_millis(100);

    fn idling() -> Self {
        Self {
            timer: Timer::new(Self::IDLE_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: MovementAnimationState::Idling,
        }
    }

    fn walking() -> Self {
        Self {
            timer: Timer::new(Self::WALKING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: MovementAnimationState::Walking,
        }
    }

    fn new() -> Self {
        Self::idling()
    }

    /// Update animation timers.
    fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.is_finished() {
            return;
        }
        self.frame = (self.frame + 1)
            % match self.state {
                MovementAnimationState::Idling => Self::IDLE_FRAMES,
                MovementAnimationState::Walking => Self::WALKING_FRAMES,
            };
    }

    /// Update animation state if it changes.
    fn update_state(&mut self, state: MovementAnimationState) {
        if self.state != state {
            match state {
                MovementAnimationState::Idling => *self = Self::idling(),
                MovementAnimationState::Walking => *self = Self::walking(),
            }
        }
    }

    /// Whether animation changed this tick.
    fn changed(&self) -> bool {
        self.timer.is_finished()
    }

    /// Return sprite index in the atlas.
    fn get_atlas_index(&self) -> usize {
        match self.state {
            MovementAnimationState::Idling => self.frame,
            MovementAnimationState::Walking => 1 + self.frame,
        }
    }

    fn get_frame(&self) -> usize {
        self.frame
    }

    fn get_state(&self) -> MovementAnimationState {
        self.state
    }
}

#[derive(Resource)]
pub struct PlayerSoundFrames(Vec<usize>);

impl SoundFrames for PlayerSoundFrames {
    fn get_frames(&self) -> &Vec<usize> {
        &self.0
    }
}
