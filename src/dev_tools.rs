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
    dev_tools::states::log_transitions, input::common_conditions::input_just_pressed, prelude::*,
};
use bevy_rapier2d::render::{DebugRenderContext, RapierDebugRenderPlugin};

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    // Add rapier debug render
    app.add_plugins(RapierDebugRenderPlugin {
        enabled: false,
        ..default()
    });

    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );
}

/// Toggle key
const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

/// Toggle debug interface
fn toggle_debug_ui(
    mut options: ResMut<UiDebugOptions>,
    mut render_context: ResMut<DebugRenderContext>,
) {
    // Toggle rapier debug context
    render_context.enabled = !render_context.enabled;

    // Toggle ui debug options
    options.toggle();
}
