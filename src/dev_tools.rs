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
use bevy_northstar::{
    debug::NorthstarDebugPlugin,
    grid::Grid,
    prelude::{DebugGrid, DebugGridBuilder, DebugOffset, OrdinalNeighborhood},
};
use bevy_rapier2d::render::{DebugRenderContext, RapierDebugRenderPlugin};

use crate::{
    levels::overworld::OverworldProcGen,
    logging::error::ERR_LOADING_TILE_DATA,
    procgen::{ProcGenerated, TileData, TileHandle},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    // Add rapier debug render
    app.add_plugins(RapierDebugRenderPlugin {
        enabled: false,
        ..default()
    });

    // Add north star debug plugin
    app.add_plugins(NorthstarDebugPlugin::<OrdinalNeighborhood>::default());

    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    // Toggle debug overlays
    app.add_systems(
        Update,
        (
            toggle_debug_ui,
            toggle_debug_colliders,
            toggle_debug_nav_grid::<OverworldProcGen>,
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

/// Toggle debug overlay for nav grids
fn toggle_debug_nav_grid<T>(
    debug_nav_grid: Option<Single<Entity, (With<DebugGrid>, Without<Grid<OrdinalNeighborhood>>)>>,
    nav_grid: Single<(Entity, &Transform), (With<Grid<OrdinalNeighborhood>>, Without<DebugGrid>)>,
    mut commands: Commands,
    data: Res<Assets<TileData<T>>>,
    handle: Res<TileHandle<T>>,
) where
    T: ProcGenerated,
{
    // Despawn debug grid and return if any exist
    if let Some(debug_nav_grid) = debug_nav_grid {
        commands.entity(debug_nav_grid.entity()).despawn();
        return;
    }

    // Get data from `TileData` with `TileHandle`
    let data = data.get(handle.0.id()).expect(ERR_LOADING_TILE_DATA);
    let tile_size = Vec2::new(data.tile_height, data.tile_width);

    // Spawn debug grid
    let (entity, transform) = nav_grid.into_inner();
    let debug = commands
        .spawn((
            DebugGridBuilder::new(tile_size.x as u32, tile_size.y as u32)
                .enable_chunks()
                .enable_entrances()
                .enable_cells()
                .build(),
            DebugOffset(transform.translation.xy().extend(0.)),
        ))
        .id();
    commands.entity(entity).add_child(debug);
}
