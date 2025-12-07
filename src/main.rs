/*
 * File: main.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 * -----
 * Heavily inspired by: https://github.com/TheBevyFlock/bevy_new_2d
 */

//! Main with [`AppPlugin`]

// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod asset_tracking;
mod audio;
mod characters;
#[cfg(feature = "dev")]
mod dev_tools;
mod menus;
mod screens;
mod theme;
mod worlds;

use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowResized};
use bevy_prng::WyRand;
use bevy_rand::plugin::EntropyPlugin;
use bevy_rapier2d::plugin::RapierPhysicsPlugin;

/// Main function
fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

/// AppPlugin that adds everything this app needs to run
struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        app.add_plugins((DefaultPlugins
            .set(AssetPlugin {
                // Wasm builds will check for meta files (that don't exist) if this isn't set.
                // This causes errors and even panics on web build on itch.
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Window {
                    title: "bevy-slime-dodge".to_string(),
                    ..default()
                }
                .into(),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),));

        // Libraries
        app.add_plugins((
            RapierPhysicsPlugin::<()>::default(),
            EntropyPlugin::<WyRand>::default(),
        ));

        // Add other plugins.
        app.add_plugins((
            asset_tracking::plugin,
            audio::plugin,
            characters::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            menus::plugin,
            screens::plugin,
            theme::plugin,
            worlds::plugin,
        ));

        // Order new `AppSystems` variants by adding them here:
        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );

        // Set up the `Pause` state.
        app.init_state::<Pause>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);

        app.add_systems(Update, fit_canvas);
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PausableSystems;

/// Camera that renders the pixel-perfect world to the [`Canvas`].
#[derive(Component)]
struct CanvasCamera;

/// Spawn [`Camera2d`]
fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d, Msaa::Off, CanvasCamera));
}

/// In-game resolution width.
const RES_WIDTH: u32 = 320;

/// In-game resolution height.
const RES_HEIGHT: u32 = 180;

/// Scales camera projection to fit the window (integer multiples only).
///
/// Source: https://bevy.org/examples/2d-rendering/pixel-grid-snap/
fn fit_canvas(
    mut resize_messages: MessageReader<WindowResized>,
    mut projection: Single<&mut Projection, With<CanvasCamera>>,
) {
    let Projection::Orthographic(projection) = &mut **projection else {
        return;
    };
    for window_resized in resize_messages.read() {
        let h_scale = window_resized.width / RES_WIDTH as f32;
        let v_scale = window_resized.height / RES_HEIGHT as f32;
        projection.scale = 1. / h_scale.min(v_scale).round();
    }
}
