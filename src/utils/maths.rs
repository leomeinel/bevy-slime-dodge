/*
 * File: maths.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

/// Quadratic ease out
///
/// Heavily inspired by: <https://easings.net/#easeOutQuad>
pub(crate) fn ease_out_quad(fraction: f32) -> f32 {
    1. - (1. - fraction) * (1. - fraction)
}
