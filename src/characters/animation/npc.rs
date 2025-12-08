/*
 * File: npc.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! Animation for npc characters

use bevy::prelude::*;
use std::time::Duration;

use crate::{
    AppSystems, PausableSystems,
    characters::{
        animation::{
            MovementAnimation, MovementAnimationState, SoundFrames, trigger_step_sound_effect,
            update_animation_atlas, update_animation_movement, update_animation_timer,
        },
        npc::SlimeAssets,
    },
};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(SlimeSoundFrames(vec![3]));
    // Animate and play sound effects based on controls.
    app.add_systems(
        Update,
        (
            update_animation_timer::<SlimeAnimation>.in_set(AppSystems::TickTimers),
            (
                update_animation_movement::<SlimeAnimation>,
                update_animation_atlas::<SlimeAnimation>,
                trigger_step_sound_effect::<SlimeAnimation, SlimeAssets, SlimeSoundFrames>,
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
pub struct SlimeAnimation {
    timer: Timer,
    frame: usize,
    state: MovementAnimationState,
}

impl MovementAnimation for SlimeAnimation {
    /// The number of idle frames.
    const IDLE_FRAMES: usize = 1;
    /// The duration of each idle frame.
    const IDLE_INTERVAL: Duration = Duration::from_millis(500);
    /// The number of walking frames.
    const WALKING_FRAMES: usize = 3;
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
pub struct SlimeSoundFrames(Vec<usize>);

impl SoundFrames for SlimeSoundFrames {
    fn get_frames(&self) -> &Vec<usize> {
        &self.0
    }
}
