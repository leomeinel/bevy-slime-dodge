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

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    asset_tracking::AssetState,
    characters::{
        CharacterAssets,
        animation::{MovementAnimation as _, player::PlayerAnimation},
    },
};

pub(super) fn plugin(app: &mut App) {
    // Add loading states via bevy_asset_loader
    app.add_loading_state(
        LoadingState::new(AssetState::AssetLoading)
            .continue_to_state(AssetState::Next)
            .load_collection::<PlayerAssets>(),
    );

    // Handle bevy_enhanced_input with input context and observers
    app.add_input_context::<Player>();
    app.add_observer(apply_movement);
    app.add_observer(stop_movement);
}

#[derive(Debug, InputAction)]
#[action_output(Vec2)]
struct Movement;

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(
        paths(
            "audio/sound-effects/movement/player-step-hard0.ogg",
            "audio/sound-effects/movement/player-step-hard1.ogg",
            "audio/sound-effects/movement/player-step-hard2.ogg"
        ),
        collection(typed)
    )]
    pub(crate) step_sounds: Vec<Handle<AudioSource>>,

    #[asset(texture_atlas_layout(tile_size_x = 24, tile_size_y = 24, columns = 9, rows = 1))]
    pub(crate) sprite_sheet: Handle<TextureAtlasLayout>,
    #[asset(image(sampler(filter = nearest)))]
    #[asset(path = "images/characters/player/male.webp")]
    pub(crate) image: Handle<Image>,
}

impl CharacterAssets for PlayerAssets {
    type Animation = PlayerAnimation;
    fn get_step_sounds(&self) -> &Vec<Handle<AudioSource>> {
        &self.step_sounds
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub(crate) struct Player;

/// The player character.
pub(crate) fn player(player_assets: &PlayerAssets) -> impl Bundle {
    let player_animation = PlayerAnimation::new();

    (
        Name::new("Player"),
        Player,
        Sprite::from_atlas_image(
            player_assets.image.clone(),
            TextureAtlas::from(player_assets.sprite_sheet.clone()),
        ),
        RigidBody::Dynamic,
        GravityScale(0.),
        Collider::cuboid(12., 12.),
        KinematicCharacterController::default(),
        player_animation,
        actions!(
            Player[(
                Action::<Movement>::new(),
                DeadZone::default(),
                SmoothNudge::default(),
                Scale::splat(120.),
                Bindings::spawn((
                    Cardinal::arrows(),
                    Cardinal::wasd_keys(),
                    Axial::left_stick(),
                )),
            )]
        ),
    )
}

/// Apply movement
fn apply_movement(
    event: On<Fire<Movement>>,
    time: Res<Time>,
    mut controller: Single<&mut KinematicCharacterController, With<Player>>,
) {
    controller.translation = Some(event.value * time.delta_secs());
}

/// Stop movement
fn stop_movement(
    _: On<Complete<Movement>>,
    mut controller: Single<&mut KinematicCharacterController, With<Player>>,
) {
    controller.translation = Some(Vec2::ZERO);
}
