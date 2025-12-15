/*
 * File: levels.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! Game worlds

pub(crate) mod overworld;

use bevy::{color::palettes::tailwind, prelude::*};

pub(super) fn plugin(app: &mut App) {
    // Add child plugins
    app.add_plugins(overworld::plugin);

    // Sort entities with `DynamicZ` by Y
    app.add_systems(PostUpdate, sort_by_y);
}

/// Color for cast shadows
pub(crate) const SHADOW_COLOR: Srgba = tailwind::GRAY_700;

/// Z-level for the level
pub(crate) const LEVEL_Z: f32 = 1.;
/// Z-level for shadows
pub(crate) const SHADOW_Z: f32 = 9.;
/// Z-level for any foreground object
pub(crate) const DEFAULT_Z: f32 = 10.;

/// Sorts entities by their y position.
/// Takes in a base value usually the sprite default Z with possibly an height offset.
/// this value could be tweaked to implement virtual Z for jumping
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub(crate) struct DynamicZ(pub(crate) f32);

/// Applies the y-sorting to the entities Z position.
///
/// Heavily inspired by: <https://github.com/fishfolk/punchy>
fn sort_by_y(mut query: Query<(&mut Transform, &DynamicZ)>) {
    for (mut transform, z_order) in query.iter_mut() {
        transform.translation.z = z_order.0 - (transform.translation.y * 0.00001);
    }
}
