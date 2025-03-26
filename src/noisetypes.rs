
// -----------------------------------------------------------------------------
// File: noisetypes.rs
// Description:
// Author(s): DIARRA Amara & SERRANO Jean-Léo
// License: CC BY-NC 4.0
// Created: March 26, 2025
// Last modified: March 26, 2025
// Version: 1.0
// -----------------------------------------------------------------------------

use std::fmt;
use clap::ValueEnum;

/// Represents different types of noise materials.
/// 
/// # Variants
/// 
/// - `Perlin`: Represents Perlin noise.
/// - `Fbm`: Represents Fractal Brownian Motion (FBM) noise.
#[derive(ValueEnum, Clone, Copy, PartialEq)]
pub enum NoiseType {
    #[value(alias = "perlin")]
    Perlin,
    #[value(alias = "fbm")]
    Fbm,
}

impl fmt::Display for NoiseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NoiseType::Perlin => write!(f, "Perlin"),
            NoiseType::Fbm => write!(f, "Fractal Brownian Motion")
        }
    }
}

