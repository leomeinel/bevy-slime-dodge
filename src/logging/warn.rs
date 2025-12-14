/*
 * File: warn.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! This stores warning messages as consts

/// Warning message if optional animation data is missing
pub(crate) const MISSING_OPTIONAL_ANIMATION_DATA: &str =
    "Missing some animation data for character.";
/// Warning message if optional asset data is missing
pub(crate) const MISSING_OPTIONAL_ASSET_DATA: &str = "Missing some assets data for character.";
/// Warning message if optional collision data is missing
pub(crate) const FALLBACK_COLLISION_DATA: &str =
    "Missing some collision data for character. Using fallback ball collider.";
