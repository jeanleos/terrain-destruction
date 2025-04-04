// -----------------------------------------------------------------------------
// File: noisegenerator.rs
// Description: This module contains the `NoiseGenerator` trait and its implementations for different noise types.
// Author(s): DIARRA Amara & SERRANO Jean-Léo
// License: CC BY-NC 4.0
// Created: April 05, 2025
// Last modified: April 05, 2025
// Version: 1.0
// -----------------------------------------------------------------------------

use noise::{NoiseFn, Perlin, Fbm, Simplex, Seedable};
use crate::noisetypes::NoiseType;

/// Represents a collection of noise generation techniques used for procedural terrain generation.
///
/// This struct aggregates multiple noise generator implementations:
/// - seed: The seed used for generating noise.
/// - fbm: An instance providing fractal brownian motion noise.
/// - simplex: An instance providing simplex noise.
/// - perlin: An instance providing Perlin noise.
/// - current_type: Tracks which type of noise is currently active.
///
/// These generators can be switched or combined to achieve a variety of noise effects.
pub struct NoiseGenerator {
    seed: u32,
    fbm: Fbm<Perlin>,
    simplex: Simplex,
    perlin: Perlin,
    current_type: NoiseType,
}

impl NoiseGenerator {
    /// Creates a new `NoiseGenerator` instance with the specified noise type.
    pub fn new(noise_type: NoiseType, seed: u32) -> Self {
        Self {
            seed,
            fbm: Fbm::new(seed),
            simplex: Simplex::new(seed),
            perlin: Perlin::new(seed),
            current_type: noise_type,
        }
    }

    /// Regenerates the current noise generator with a new seed.
    pub fn generate(&mut self, seed: u32) {
        self.seed = seed;
        match self.current_type {
            NoiseType::Fbm => self.fbm = Fbm::new(self.seed).set_seed(seed),
            NoiseType::Perlin => self.perlin = Perlin::new(self.seed).set_seed(seed),
            NoiseType::Simplex => self.simplex = Simplex::new(self.seed).set_seed(seed),
        }
    }

    /// Gets the noise value for the given `[x, y]` coordinates from the current noise type.
    pub fn get(&self, x: f64, y: f64) -> f64 {
        match self.current_type {
            NoiseType::Fbm => self.fbm.get([x, y]),
            NoiseType::Perlin => self.perlin.get([x, y]),
            NoiseType::Simplex => self.simplex.get([x, y]),
        }
    }

    /// Sets the current noise type.
    pub fn set_noise_type(&mut self, noise_type: NoiseType) {
        self.current_type = noise_type;
    }
}

