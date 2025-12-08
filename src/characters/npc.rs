/*
 * File: npc.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! Npc-specific behavior.

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    asset_tracking::AssetState,
    characters::{
        CharacterAssets,
        animation::{MovementAnimation as _, npc::SlimeAnimation},
    },
};

/// Plugin
pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(AssetState::AssetLoading)
            .continue_to_state(AssetState::Next)
            .load_collection::<SlimeAssets>(),
    );
}

#[derive(AssetCollection, Resource)]
pub(crate) struct SlimeAssets {
    #[asset(paths("audio/sound-effects/movement/bounce.ogg"), collection(typed))]
    pub(crate) step_sounds: Vec<Handle<AudioSource>>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 3, rows = 1))]
    pub(crate) sprite_sheet: Handle<TextureAtlasLayout>,
    #[asset(image(sampler(filter = nearest)))]
    #[asset(path = "images/characters/npc/slime.webp")]
    pub(crate) image: Handle<Image>,
}

impl CharacterAssets for SlimeAssets {
    type Animation = SlimeAnimation;
    fn get_step_sounds(&self) -> &Vec<Handle<AudioSource>> {
        &self.step_sounds
    }
}

#[derive(Component)]
pub(crate) struct Npc;

#[derive(Component)]
pub(crate) struct Slime;

/// The slime enemy.
pub(crate) fn slime(slime_assets: &SlimeAssets) -> impl Bundle {
    let player_animation = SlimeAnimation::new();

    (
        Name::new("Player"),
        Npc,
        Slime,
        Sprite::from_atlas_image(
            slime_assets.image.clone(),
            TextureAtlas::from(slime_assets.sprite_sheet.clone()),
        ),
        RigidBody::Dynamic,
        GravityScale(0.),
        Collider::ball(8.),
        KinematicCharacterController::default(),
        player_animation,
    )
}
