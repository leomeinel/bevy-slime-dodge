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
    characters::Character,
    levels::{Level, overworld::OverworldProcGen},
    logging::error::{ERR_INVALID_MINIMUM_CHUNK_POS, ERR_LOADING_TILE_DATA},
    procgen::{
        CHUNK_SIZE, PROCGEN_DISTANCE, ProcGenController, ProcGenSpawned, ProcGenTimer,
        ProcGenerated, TileData, TileHandle,
    },
};

pub(super) fn plugin(app: &mut App) {
    // Add north star plugin
    app.add_plugins(NorthstarPlugin::<OrdinalNeighborhood>::default());
}

/// Size of the [`Grid<OrdinalNeighborhood>`]
const GRID_SIZE: UVec2 = UVec2::new(
    CHUNK_SIZE.x * (PROCGEN_DISTANCE as u32 * 2 + 1),
    CHUNK_SIZE.y * (PROCGEN_DISTANCE as u32 * 2 + 1),
);

/// Replace [`Grid<OrdinalNeighborhood>`] with new grid at correct world position
///
/// ## Traits
///
/// - `T` must implement [`ProcGenerated`] and is used as a level's procedurally generated item.
pub(crate) fn spawn_nav_grid<T>(
    level: Single<Entity, (With<T>, Without<Grid<OrdinalNeighborhood>>)>,
    mut commands: Commands,
) where
    T: Level,
{
    let grid_settings = GridSettingsBuilder::new_2d(GRID_SIZE.x, GRID_SIZE.y)
        .chunk_size(CHUNK_SIZE.x)
        .default_impassable()
        .enable_collision()
        .build();
    let entity = commands
        .spawn(Grid::<OrdinalNeighborhood>::new(&grid_settings))
        .id();

    commands.entity(level.entity()).add_child(entity);
}

// FIXME: This is quite heavy. Try to reduce load or spread load.
/// Rebuild the nav grid
///
/// Currently this sets every cell to [Nav::Passable], but this can in the future also include obstacle detection.
pub(crate) fn rebuild_nav_grid(
    _: On<ProcGenSpawned<OverworldProcGen>>,
    mut grid: Single<&mut Grid<OrdinalNeighborhood>>,
) {
    // Set every cell to passable
    for x in 0..GRID_SIZE.x {
        for y in 0..GRID_SIZE.y {
            grid.set_nav(UVec3::new(x, y, 0), Nav::Passable(1));
        }
    }

    // Rebuild grid
    grid.build();
}

// FIXME: Maybe do not use a timer and do not loop through all characters.
/// Update nav grid position of [`Character`]
///
/// ## Traits
///
/// - `T` must implement '[`Character`]'.
/// - `A` must implement [`ProcGenerated`] and is used as a level's procedurally generated item.
pub(crate) fn update_nav_grid_agent_pos<T, A>(
    characters: Query<(Entity, &Transform, Ref<Transform>), With<T>>,
    mut commands: Commands,
    controller: Res<ProcGenController<A>>,
    data: Res<Assets<TileData<A>>>,
    handle: Res<TileHandle<A>>,
    timer: Res<ProcGenTimer>,
) where
    T: Character,
    A: ProcGenerated,
{
    // Return if timer has not finished
    if !timer.0.just_finished() {
        return;
    }
    // Return if controller positions are empty.
    if controller.positions.is_empty() {
        return;
    }

    // Get data from `TileData` with `TileHandle`
    let data = data.get(handle.0.id()).expect(ERR_LOADING_TILE_DATA);
    let tile_size = Vec2::new(data.tile_height, data.tile_width);

    // Determine minimum chunk position
    let min_chunk_pos = controller
        .positions
        .values()
        .min_by_key(|pos| (pos.x, pos.y))
        .expect(ERR_INVALID_MINIMUM_CHUNK_POS);

    for (entity, transform, ref_transform) in characters {
        // Continue if transform is not changed
        if !ref_transform.is_changed() {
            continue;
        }

        // Insert agent pos
        let pos = UVec2::new(
            (transform.translation.x / tile_size.x - min_chunk_pos.x as f32 * CHUNK_SIZE.x as f32)
                .floor() as u32,
            (transform.translation.y / tile_size.y - min_chunk_pos.y as f32 * CHUNK_SIZE.x as f32)
                .floor() as u32,
        );
        commands.entity(entity).insert(AgentPos(pos.extend(0)));
    }
}
