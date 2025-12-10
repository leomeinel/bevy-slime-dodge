/*
 * File: characters.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! Characters

pub(crate) mod animations;
pub(crate) mod npc;
pub(crate) mod player;

use std::marker::PhantomData;

use bevy::{prelude::*, reflect::Reflectable};
use bevy_rapier2d::prelude::Collider;

pub(super) fn plugin(app: &mut App) {
    // Add child plugins
    app.add_plugins((animations::plugin, npc::plugin, player::plugin));
}

/// Applies to anything that stores character assets
pub(crate) trait CharacterAssets {
    fn get_step_sounds(&self) -> &Vec<Handle<AudioSource>>;
    fn get_image(&self) -> &Handle<Image>;
}
#[macro_export]
macro_rules! impl_character_assets {
    ($type: ty) => {
        impl CharacterAssets for $type {
            fn get_step_sounds(&self) -> &Vec<Handle<AudioSource>> {
                &self.step_sounds
            }
            fn get_image(&self) -> &Handle<Image> {
                &self.image
            }
        }
    };
}

/// Animation data deserialized from a ron file as a generic
#[derive(serde::Deserialize, Asset, TypePath)]
pub(crate) struct CollisionData<T>
where
    T: Reflectable,
{
    shape: String,
    width: f32,
    height: f32,
    #[serde(skip)]
    _phantom: PhantomData<T>,
}

/// Handle for [`CollisionData`] as a generic
#[derive(Resource)]
pub(crate) struct CollisionHandle<T>(Handle<CollisionData<T>>)
where
    T: Reflectable;

/// Get collsion vertices
pub(crate) fn get_collider<T>(
    data: &Res<Assets<CollisionData<T>>>,
    handle: &Res<CollisionHandle<T>>,
) -> Collider
where
    T: Component + Default + Reflectable,
{
    // Get data from `CollisionData` with `CollisionHandle`
    let data = data.get(handle.0.id()).unwrap();

    match data.shape.as_str() {
        "ball" => Collider::ball(data.width / 2.),
        "capsule_x" => Collider::capsule_x(data.height / 6., data.width / 2.),
        "capsule_y" => Collider::capsule_y(data.height / 6., data.width / 2.),
        _ => Collider::cuboid(data.width / 2., data.height / 2.),
    }
}
