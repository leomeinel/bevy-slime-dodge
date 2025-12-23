/*
 * File: dev_tools.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 * -----
 * Heavily inspired by: https://github.com/TheBevyFlock/bevy_new_2d
 */

//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    color::palettes::tailwind, dev_tools::states::log_transitions,
    input::common_conditions::input_just_pressed, prelude::*,
};
use bevy_rapier2d::render::{DebugRenderContext, RapierDebugRenderPlugin};
use vleue_navigator::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    // Add rapier debug render
    app.add_plugins(RapierDebugRenderPlugin {
        enabled: false,
        ..default()
    });

    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    // Toggle debug overlays
    app.add_systems(
        Update,
        (
            toggle_debug_ui,
            toggle_debug_colliders,
            toggle_debug_navmeshes,
        )
            .run_if(input_just_pressed(TOGGLE_KEY)),
    );
}

/// Toggle key
const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

/// Toggle debug overlay for UI
fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    // Toggle ui debug options
    options.toggle();
}

/// Toggle debug overlay for rapier colliders
fn toggle_debug_colliders(mut render_context: ResMut<DebugRenderContext>) {
    // Toggle rapier debug context
    render_context.enabled = !render_context.enabled;
}

/// rgb(219, 39, 119)
const NAVMESH_DEBUG_COLOR: Srgba = tailwind::PINK_600;

/// Toggle debug overlay for navmeshes
fn toggle_debug_navmeshes(
    debug_navmeshes: Query<Entity, With<NavMeshDebug>>,
    live_navmeshes: Query<Entity, With<ManagedNavMesh>>,
    mut commands: Commands,
) {
    // Despawn debug meshes and return if any exist
    if !debug_navmeshes.is_empty() {
        for entity in &debug_navmeshes {
            commands.entity(entity).remove::<NavMeshDebug>();
        }
        return;
    }

    // Spawn debug navmeshes
    for entity in &live_navmeshes {
        commands
            .entity(entity)
            .insert(NavMeshDebug(NAVMESH_DEBUG_COLOR.into()));
    }
}
