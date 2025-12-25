/*
 * File: navigation.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 * -----
 * Heavily inspired by: https://github.com/JtotheThree/bevy_northstar
 */

use bevy::prelude::*;
use bevy_northstar::prelude::*;

use crate::{
    levels::Level,
    logging::error::{ERR_INVALID_MINIMUM_CHUNK_POS, ERR_LOADING_TILE_DATA},
    procgen::{
        CHUNK_SIZE, PROCGEN_DISTANCE, ProcGenController, ProcGenTimer, ProcGenerated, TileData,
        TileHandle,
    },
};

pub(super) fn plugin(app: &mut App) {
    // Add north star plugin
    app.add_plugins(NorthstarPlugin::<OrdinalNeighborhood>::default());
}

/// Replace [`Grid<OrdinalNeighborhood>`] with new grid at correct world position
pub(crate) fn spawn_nav_grid<T, A>(
    grid: Option<Single<Entity, (With<Grid<OrdinalNeighborhood>>, Without<A>)>>,
    level: Single<Entity, (With<A>, Without<Grid<OrdinalNeighborhood>>)>,
    mut commands: Commands,
    controller: Res<ProcGenController<T>>,
    data: Res<Assets<TileData<T>>>,
    handle: Res<TileHandle<T>>,
    timer: Res<ProcGenTimer>,
) where
    T: ProcGenerated,
    A: Level,
{
    // Return if timer has not finished
    if !timer.0.just_finished() {
        return;
    }
    // Return if `controller` has not changed
    if !controller.is_changed() {
        return;
    }

    // Despawn outdated grid if one exists
    if let Some(grid) = grid {
        commands.entity(grid.entity()).despawn();
    }

    // Get data from `TileData` with `TileHandle`
    let data = data.get(handle.0.id()).expect(ERR_LOADING_TILE_DATA);
    let tile_size = Vec2::new(data.tile_height, data.tile_width);

    // Determine spawn position and spawn nav grid
    let min_chunk_pos = controller
        .positions
        .values()
        .min_by_key(|pos| (pos.x, pos.y))
        .expect(ERR_INVALID_MINIMUM_CHUNK_POS);
    let world_pos = Vec2::new(
        min_chunk_pos.x as f32 * CHUNK_SIZE.x as f32 * tile_size.x,
        min_chunk_pos.y as f32 * CHUNK_SIZE.y as f32 * tile_size.y,
    );
    // Add entity to level so that level handles despawning
    let entity = commands.spawn(nav_grid(world_pos)).id();
    commands.entity(level.entity()).add_child(entity);
}

/// Size of the [`Grid<OrdinalNeighborhood>`]
const GRID_SIZE: UVec2 = UVec2::new(
    CHUNK_SIZE.x * (PROCGEN_DISTANCE as u32 * 2 + 1),
    CHUNK_SIZE.y * (PROCGEN_DISTANCE as u32 * 2 + 1),
);

/// Bundle containing the [`Grid<OrdinalNeighborhood>`] for the map
fn nav_grid(world_pos: Vec2) -> impl Bundle {
    let grid_settings = GridSettingsBuilder::new_2d(GRID_SIZE.x, GRID_SIZE.y)
        .chunk_size(CHUNK_SIZE.x)
        .build();
    let grid = Grid::<OrdinalNeighborhood>::new(&grid_settings);
    (grid, Transform::from_translation(world_pos.extend(0.)))
}
