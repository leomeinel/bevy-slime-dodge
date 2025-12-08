/*
 * File: interaction.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 * -----
 * Heavily inspired by: https://github.com/TheBevyFlock/bevy_new_2d
 */

use crate::{asset_tracking::AssetState, audio::sound_effect};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // Add loading states via bevy_asset_loader
    app.add_loading_state(
        LoadingState::new(AssetState::AssetLoading)
            .continue_to_state(AssetState::Next)
            .load_collection::<InteractionAssets>(),
    );

    // Visualize ui interactions with color palette
    app.add_systems(Update, apply_interaction_palette);

    // Play sound effects
    app.add_observer(play_on_hover_sound_effect);
    app.add_observer(play_on_click_sound_effect);
}

/// Palette for widget interactions. Add this to an entity that supports
/// [`Interaction`]s, such as a button, to change its [`BackgroundColor`] based
/// on the current interaction state.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

fn apply_interaction_palette(
    mut query: Query<
        (&Interaction, &InteractionPalette, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, palette, mut background) in &mut query {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        }
        .into();
    }
}

#[derive(AssetCollection, Resource)]
struct InteractionAssets {
    #[asset(path = "audio/sound-effects/ui/hover.ogg")]
    hover: Handle<AudioSource>,
    #[asset(path = "audio/sound-effects/ui/click.ogg")]
    click: Handle<AudioSource>,
}

fn play_on_hover_sound_effect(
    event: On<Pointer<Over>>,
    query: Query<(), With<Interaction>>,
    mut commands: Commands,
    interaction_assets: Option<Res<InteractionAssets>>,
) {
    let Some(interaction_assets) = interaction_assets else {
        return;
    };

    if query.contains(event.entity) {
        commands.spawn(sound_effect(interaction_assets.hover.clone()));
    }
}

fn play_on_click_sound_effect(
    event: On<Pointer<Click>>,
    query: Query<(), With<Interaction>>,
    mut commands: Commands,
    interaction_assets: Option<Res<InteractionAssets>>,
) {
    let Some(interaction_assets) = interaction_assets else {
        return;
    };

    if query.contains(event.entity) {
        commands.spawn(sound_effect(interaction_assets.click.clone()));
    }
}
