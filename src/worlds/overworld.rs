/*
 * File: overworld.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! Overworld-specific behavior.

use bevy::{color::palettes::tailwind, prelude::*};

use crate::{
    asset_tracking::LoadResource,
    audio::music,
    characters::player::{PlayerAssets, PlayerSpriteSheet, player},
    screens::Screen,
};

/// Plugin
pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/bit-bit-loop.ogg"),
        }
    }
}

// rgb(107, 114, 128)
const GROUND_COLOR: Srgba = tailwind::GRAY_500;

pub(crate) fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    player_assets: Res<PlayerAssets>,
    player_spritesheet: Res<PlayerSpriteSheet>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Name::new("Level"),
        Mesh2d(meshes.add(Rectangle::new(1000., 700.))),
        MeshMaterial2d(materials.add(Into::<Color>::into(GROUND_COLOR))),
        Transform::from_xyz(0., 0., 2.),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
        children![
            player(&player_assets, &player_spritesheet),
            (
                Name::new("Gameplay Music"),
                music(level_assets.music.clone())
            )
        ],
    ));
}
