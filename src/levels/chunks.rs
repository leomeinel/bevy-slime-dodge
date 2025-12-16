/*
 * File: chunks.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use std::marker::PhantomData;

use bevy::{platform::collections::HashSet, prelude::*, reflect::Reflectable};
use bevy_ecs_tilemap::prelude::*;

use crate::{
    CanvasCamera, RES_HEIGHT,
    levels::{LEVEL_Z, LevelAssets},
    logging::warn::LEVEL_MISSING_OPTIONAL_TILE_DATA,
    screens::Screen,
};

/// Animation data deserialized from a ron file as a generic
#[derive(serde::Deserialize, Asset, TypePath, Default)]
pub(crate) struct TileData<T>
where
    T: Reflectable,
{
    tile_width: f32,
    tile_height: f32,
    #[serde(default)]
    full_dirt_tiles: Option<HashSet<(u32, u32)>>,
    #[serde(default)]
    full_grass_tiles: Option<HashSet<(u32, u32)>>,
    #[serde(default)]
    corner_outer_grass_to_dirt_tiles: Option<HashSet<(u32, u32)>>,
    #[serde(default)]
    corner_outer_dirt_to_grass_tiles: Option<HashSet<(u32, u32)>>,
    #[serde(default)]
    side_dirt_and_grass_tiles: Option<HashSet<(u32, u32)>>,
    #[serde(default)]
    diag_stripe_grass_in_dirt_tiles: Option<HashSet<(u32, u32)>>,
    #[serde(skip)]
    _phantom: PhantomData<T>,
}
impl<T> TileData<T>
where
    T: Reflectable,
{
    fn get_tiles(
        &self,
    ) -> Option<(
        HashSet<(u32, u32)>,
        HashSet<(u32, u32)>,
        HashSet<(u32, u32)>,
        HashSet<(u32, u32)>,
        HashSet<(u32, u32)>,
        HashSet<(u32, u32)>,
    )> {
        Some((
            self.full_dirt_tiles.as_ref().cloned()?,
            self.full_grass_tiles.as_ref().cloned()?,
            self.corner_outer_grass_to_dirt_tiles.as_ref().cloned()?,
            self.corner_outer_dirt_to_grass_tiles.as_ref().cloned()?,
            self.side_dirt_and_grass_tiles.as_ref().cloned()?,
            self.diag_stripe_grass_in_dirt_tiles.as_ref().cloned()?,
        ))
    }
}

/// Handle for [`TileData`] as a generic
#[derive(Resource)]
pub(crate) struct TileHandle<T>(pub(crate) Handle<TileData<T>>)
where
    T: Reflectable;

/// Chunk controller that stores spawned chunks
#[derive(Default, Debug, Resource)]
pub(crate) struct ChunkController<T> {
    pub(crate) chunks: HashSet<IVec2>,
    _phantom: PhantomData<T>,
}

/// Chunk marker
#[derive(Component)]
pub(crate) struct Chunk;

/// Size of a single chunk
const CHUNK_SIZE: UVec2 = UVec2 { x: 16, y: 16 };
/// Chunk size for [`TilemapRenderSettings`]
const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};
/// Render distance of chunks
const RENDER_DISTANCE: i32 = 2;
/// Despawn range of chunks
const DESPAWN_RANGE: f32 = RES_HEIGHT * 4.;

/// Spawn chunks around the [`CanvasCamera`]
pub(crate) fn spawn_chunks<T, A>(
    camera: Single<&Transform, (With<CanvasCamera>, Without<Chunk>)>,
    mut commands: Commands,
    mut controller: ResMut<ChunkController<T>>,
    data: Res<Assets<TileData<T>>>,
    handle: Res<TileHandle<T>>,
    assets: If<Res<A>>,
) where
    T: Component + Default + Reflectable,
    A: LevelAssets + Resource,
{
    // Get data from `TileData` with `TileHandle`
    let data = data.get(handle.0.id()).unwrap();
    let tile_size = TilemapTileSize {
        x: data.tile_height,
        y: data.tile_width,
    };
    // FIXME: Use this for conditional spawning/arranging
    let Some(_tiles) = data.get_tiles() else {
        // Return and do not spawn chunks if tiles are not configured correctly
        warn_once!("{}", LEVEL_MISSING_OPTIONAL_TILE_DATA);
        return;
    };

    // Get target translation for new chunk from camera translation
    let camera_pos_ivec2 = &camera.translation.xy().as_ivec2();
    let chunk_size_ivec2 = IVec2::new(CHUNK_SIZE.x as i32, CHUNK_SIZE.y as i32);
    let tile_size_ivec2 = IVec2::new(tile_size.x as i32, tile_size.y as i32);
    let chunk_pos = camera_pos_ivec2 / (chunk_size_ivec2 * tile_size_ivec2);

    // Spawn chunk behind and in front of chunk position if it does not contain a chunk already
    for y in (chunk_pos.y - RENDER_DISTANCE)..(chunk_pos.y + RENDER_DISTANCE) {
        for x in (chunk_pos.x - RENDER_DISTANCE)..(chunk_pos.x + RENDER_DISTANCE) {
            if !controller.chunks.contains(&IVec2::new(x, y)) {
                controller.chunks.insert(IVec2::new(x, y));
                spawn_chunk::<A>(
                    &mut commands,
                    &assets.0,
                    IVec2::new(x, y),
                    tile_size,
                    TileTextureIndex(8),
                );
            }
        }
    }
}

/// Despawn chunks
///
/// This removes the coordinates from [`ChunkController<T>`] and despawns the entity.
pub(crate) fn despawn_chunks<T>(
    camera: Single<&Transform, (With<CanvasCamera>, Without<Chunk>)>,
    query: Query<(Entity, &Transform), (With<Chunk>, Without<CanvasCamera>, Without<T>)>,
    mut commands: Commands,
    mut controller: ResMut<ChunkController<T>>,
    data: Res<Assets<TileData<T>>>,
    handle: Res<TileHandle<T>>,
) where
    T: Component + Default + Reflectable,
{
    // Get data from `TileData` with `TileHandle`
    let data = data.get(handle.0.id()).unwrap();
    let tile_size = TilemapTileSize {
        x: data.tile_height,
        y: data.tile_width,
    };

    // Despawn chunks outside of `DESPAWN_RANGE`
    for (entity, chunk) in query.iter() {
        let chunk_pos = chunk.translation.xy();
        let distance = camera.translation.xy().distance(chunk_pos);

        if distance > DESPAWN_RANGE {
            let x = (chunk_pos.x / (CHUNK_SIZE.x as f32 * tile_size.x)).floor() as i32;
            let y = (chunk_pos.y / (CHUNK_SIZE.y as f32 * tile_size.y)).floor() as i32;
            controller.chunks.remove(&IVec2::new(x, y));
            commands.entity(entity).despawn();
        }
    }
}

pub(crate) fn delete_chunks<T>(mut controller: ResMut<ChunkController<T>>)
where
    T: Component + Default + Reflectable,
{
    controller.chunks.clear();
}

/// Spawn a single chunk
fn spawn_chunk<A>(
    commands: &mut Commands,
    assets: &Res<A>,
    chunk_pos: IVec2,
    tile_size: TilemapTileSize,
    texture_index: TileTextureIndex,
) where
    A: LevelAssets + Resource,
{
    // Create empty entity and storage dedicated to this chunk
    let container_entity = commands.spawn(DespawnOnExit(Screen::Gameplay)).id();
    let mut storage = TileStorage::empty(CHUNK_SIZE.into());

    // Spawn a `TileBundle` mapped to the container entity for each x/y in `CHUNK_SIZE`,
    // add as child to container entity and add to storage.
    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            let position = TilePos { x, y };
            let entity = commands
                .spawn((
                    Chunk,
                    TileBundle {
                        position,
                        texture_index,
                        tilemap_id: TilemapId(container_entity),
                        ..default()
                    },
                ))
                .id();
            commands.entity(container_entity).add_child(entity);
            storage.set(&position, entity);
        }
    }

    let transform = Transform::from_xyz(
        chunk_pos.x as f32 * CHUNK_SIZE.x as f32 * tile_size.x,
        chunk_pos.y as f32 * CHUNK_SIZE.y as f32 * tile_size.y,
        LEVEL_Z,
    );
    let handle = assets.get_tile_set().clone();

    // Insert TileMapBundle with storage, transform and texture from handle to container entity
    commands.entity(container_entity).insert(TilemapBundle {
        grid_size: tile_size.into(),
        size: CHUNK_SIZE.into(),
        storage,
        texture: TilemapTexture::Single(handle),
        tile_size,
        transform,
        render_settings: TilemapRenderSettings {
            render_chunk_size: RENDER_CHUNK_SIZE,
            y_sort: false,
        },
        ..default()
    });
}
