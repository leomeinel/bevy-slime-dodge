/*
 * File: characters.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! Characters

pub(crate) mod animation;
pub(crate) mod npc;
pub(crate) mod player;

use bevy::prelude::*;

use crate::characters::animation::MovementAnimation;

pub(super) fn plugin(app: &mut App) {
    // Add child plugins
    app.add_plugins((animation::plugin, npc::plugin, player::plugin));
}

pub(crate) trait CharacterAssets {
    type Animation: MovementAnimation;
    fn get_step_sounds(&self) -> &Vec<Handle<AudioSource>>;
}
