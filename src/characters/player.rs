/*
 * File: player.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 * -----
 * Heavily inspired by: https://github.com/TheBevyFlock/bevy_new_2d
 */

//! Player-specific behavior.

use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};
use bevy_rapier2d::prelude::*;

use crate::{
    AppSystems, PausableSystems, asset_tracking::LoadResource,
    characters::animation::PlayerAnimation,
};

/// Plugin
pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();
    app.load_resource::<PlayerSpriteSheet>();

    app.add_systems(
        Update,
        record_player_directional_input
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

/// The player character.
pub fn player(
    player_assets: &PlayerAssets,
    player_sprite_sheet: &PlayerSpriteSheet,
) -> impl Bundle {
    let player_animation = PlayerAnimation::new();

    (
        Name::new("Player"),
        Player,
        Sprite {
            image: player_assets.image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: player_sprite_sheet.0.clone(),
                ..default()
            }),
            ..default()
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(12., 12.),
        KinematicCharacterController::default(),
        player_animation,
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct Player;

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut controller_q: Single<&mut KinematicCharacterController, With<Player>>,
) {
    let velocity = 100.0 * time.delta_secs();
    let mut intent = Vec2::ZERO;

    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y += velocity;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y -= velocity;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= velocity;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += velocity;
    }

    controller_q.translation = Some(intent);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub(crate) struct PlayerSpriteSheet(Handle<TextureAtlasLayout>);

impl FromWorld for PlayerSpriteSheet {
    // Source: https://taintedcoders.com/bevy/sprites
    fn from_world(world: &mut World) -> Self {
        let texture_atlas = TextureAtlasLayout::from_grid((24, 24).into(), 9, 1, None, None);
        let mut texture_atlases = world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        Self(texture_atlas_handle)
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    image: Handle<Image>,
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        let image: Handle<Image> = assets.load_with_settings(
            "images/characters/player/male.webp",
            |settings: &mut ImageLoaderSettings| {
                // Use `nearest` image sampling to preserve pixel art style.
                settings.sampler = ImageSampler::nearest();
            },
        );

        Self {
            image,
            steps: vec![assets.load("audio/sound-effects/step/stone01.ogg")],
        }
    }
}
