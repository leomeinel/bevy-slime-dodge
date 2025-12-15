/*
 * File: arena.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! Arena-specific behavior.

use std::{f32::consts::FRAC_1_SQRT_2, ops::Range};

use bevy::{color::palettes::tailwind, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_prng::WyRand;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{
    animations::{AnimationRng, Animations},
    audio::music,
    characters::{
        CollisionData, CollisionHandle, VisualMap,
        npc::{Slime, slime, slime_visual},
        player::{Player, player, player_visual},
    },
    levels::{DEFAULT_Z, DynamicZ, LEVEL_Z, SHADOW_COLOR, SHADOW_Z},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    // Initialize asset state
    app.init_state::<ArenaAssetState>();

    // Add loading states via bevy_asset_loader
    app.add_loading_state(
        LoadingState::new(ArenaAssetState::AssetLoading)
            .continue_to_state(ArenaAssetState::Next)
            .load_collection::<ArenaAssets>(),
    );
}

/// Asset state that tracks asset loading
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum ArenaAssetState {
    #[default]
    AssetLoading,
    Next,
}

/// Assets for the arena
#[derive(AssetCollection, Resource)]
pub(crate) struct ArenaAssets {
    #[asset(path = "audio/music/bit-bit-loop.ogg")]
    music: Handle<AudioSource>,
}

/// rgb(107, 114, 128)
const GROUND_COLOR: Srgba = tailwind::GRAY_500;
/// Width and height of the ground
const GROUND_WIDTH_HEIGHT: f32 = 640.;

/// Level position
const LEVEL_POS: Vec3 = Vec3::new(0., 0., LEVEL_Z);

/// rgb(17, 24, 39)
const BORDER_COLOR: Srgba = tailwind::GRAY_900;
/// Border height
const BORDER_HEIGHT: f32 = 20.;
/// 90 degree angle using only const functions
const QUAT_Z_90: Quat = Quat::from_xyzw(0., 0., FRAC_1_SQRT_2, FRAC_1_SQRT_2);
/// Border transforms
const BORDER_TRANSFORMS: [Transform; 4] = [
    Transform {
        translation: Vec3::new(GROUND_WIDTH_HEIGHT / 2. + BORDER_HEIGHT / 2., 0., DEFAULT_Z),
        rotation: QUAT_Z_90,
        scale: Vec3::ONE,
    },
    Transform {
        translation: Vec3::new(
            -GROUND_WIDTH_HEIGHT / 2. - BORDER_HEIGHT / 2.,
            0.,
            DEFAULT_Z,
        ),
        rotation: QUAT_Z_90,
        scale: Vec3::ONE,
    },
    Transform {
        translation: Vec3::new(0., GROUND_WIDTH_HEIGHT / 2. + BORDER_HEIGHT / 2., DEFAULT_Z),
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    },
    Transform {
        translation: Vec3::new(
            0.,
            -GROUND_WIDTH_HEIGHT / 2. - BORDER_HEIGHT / 2.,
            DEFAULT_Z,
        ),
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    },
];

/// Slime positions
const SLIME_POSITIONS: [Vec3; 4] = [
    Vec3::new(40., 0., DEFAULT_Z),
    Vec3::new(-40., 0., DEFAULT_Z),
    Vec3::new(0., 40., DEFAULT_Z),
    Vec3::new(0., -40., DEFAULT_Z),
];
/// Slime animation delay
const SLIME_ANIMATION_DELAY: Range<f32> = 1.0..10.0;

/// Player position
const PLAYER_POS: Vec3 = Vec3::new(0., 0., DEFAULT_Z);
/// Player animation delay
const PLAYER_ANIMATION_DELAY: Range<f32> = 1.0..5.0;

/// Spawn arena with player, enemies and objects
pub(crate) fn spawn_arena(
    mut animation_rng: Single<&mut WyRand, With<AnimationRng>>,
    mut commands: Commands,
    mut visual_map: ResMut<VisualMap>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    level_assets: Res<ArenaAssets>,
    player_animations: Res<Animations<Player>>,
    player_collision_data: Res<Assets<CollisionData<Player>>>,
    player_collision_handle: Res<CollisionHandle<Player>>,
    slime_animations: Res<Animations<Slime>>,
    slime_collision_data: Res<Assets<CollisionData<Slime>>>,
    slime_collision_handle: Res<CollisionHandle<Slime>>,
) {
    let level = commands
        .spawn((
            Name::new("Level"),
            Mesh2d(meshes.add(Rectangle::new(GROUND_WIDTH_HEIGHT, GROUND_WIDTH_HEIGHT))),
            MeshMaterial2d(materials.add(Into::<Color>::into(GROUND_COLOR))),
            Transform::from_translation(LEVEL_POS),
            Visibility::default(),
            DespawnOnExit(Screen::Gameplay),
            children![(
                Name::new("Gameplay Music"),
                music(level_assets.music.clone())
            ),],
        ))
        .id();

    for transform in BORDER_TRANSFORMS {
        commands.entity(level).with_children(|commands| {
            commands.spawn((
                DynamicZ(DEFAULT_Z),
                transform,
                border(&mut meshes, &mut materials),
            ));
        });
    }

    for pos in SLIME_POSITIONS {
        commands.entity(level).with_children(|commands_p| {
            let slime = commands_p
                .spawn((
                    Visibility::Inherited,
                    DynamicZ(DEFAULT_Z),
                    Transform::from_translation(pos),
                    slime(&slime_collision_data, &slime_collision_handle),
                ))
                .id();
            commands_p
                .commands()
                .entity(slime)
                .with_children(|commands_c| {
                    let slime_visual = commands_c
                        .spawn((
                            DynamicZ(DEFAULT_Z),
                            slime_visual(
                                &slime_animations,
                                animation_rng.random_range(SLIME_ANIMATION_DELAY),
                            ),
                        ))
                        .id();
                    visual_map.0.insert(slime, slime_visual);
                });
            commands_p
                .commands()
                .entity(slime)
                .with_children(|commands_c| {
                    commands_c.spawn((
                        DynamicZ(SHADOW_Z),
                        Transform::from_xyz(0., -8., SHADOW_Z),
                        Mesh2d(meshes.add(Circle::new(4.))),
                        MeshMaterial2d(materials.add(Color::from(SHADOW_COLOR.with_alpha(0.25)))),
                    ));
                });
        });
    }

    commands.entity(level).with_children(|commands_p| {
        let player = commands_p
            .spawn((
                DynamicZ(DEFAULT_Z),
                Visibility::Inherited,
                Transform::from_translation(PLAYER_POS),
                player(&player_collision_data, &player_collision_handle),
            ))
            .id();
        commands_p
            .commands()
            .entity(player)
            .with_children(|commands_c| {
                let player_visual = commands_c
                    .spawn((
                        DynamicZ(DEFAULT_Z),
                        player_visual(
                            &player_animations,
                            animation_rng.random_range(PLAYER_ANIMATION_DELAY),
                        ),
                    ))
                    .id();
                visual_map.0.insert(player, player_visual);
            });
        commands_p
            .commands()
            .entity(player)
            .with_children(|commands_c| {
                commands_c.spawn((
                    DynamicZ(SHADOW_Z),
                    Transform::from_xyz(0., -9., SHADOW_Z),
                    Mesh2d(meshes.add(Circle::new(4.5))),
                    MeshMaterial2d(materials.add(Color::from(SHADOW_COLOR.with_alpha(0.25)))),
                ));
            });
    });
}

/// Border for the arena
fn border(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> impl Bundle {
    (
        RigidBody::Fixed,
        Collider::cuboid(
            (GROUND_WIDTH_HEIGHT + BORDER_HEIGHT * 2.) / 2.,
            BORDER_HEIGHT / 2.,
        ),
        Mesh2d(meshes.add(Rectangle::new(
            GROUND_WIDTH_HEIGHT + BORDER_HEIGHT * 2.,
            BORDER_HEIGHT,
        ))),
        MeshMaterial2d(materials.add(Color::from(BORDER_COLOR))),
    )
}
