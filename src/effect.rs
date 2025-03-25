// -----------------------------------------------------------------------------
// File: effects.rs
// Description: Defines visual effects and their properties.
// Author(s): DIARRA Amara & SERRANO Jean-Léo
// License: CC BY-NC 4.0
// Created: March 15, 2025
// Last modified: March 25, 2025
// Version: 1.0
// -----------------------------------------------------------------------------

use std::time::Instant;

/// Represents the type of visual effect in the game.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EffectType {
    /// A bubble effect.
    Bubbles,
    /// A more intense bubble effect.
    MoreBubbles,
    /// A lightning effect.
    Lightning,
}

/// Represents a visual effect in the game, including its type, position, direction, 
/// and other properties related to its behaviour.
#[derive(Debug)]
pub struct Effect {
    /// The type of the effect (e.g., bubbles, lightning).
    pub effect_type: EffectType,
    /// The position of the effect in the game world, represented as (x, y) coordinates.
    pub position: (f32, f32),
    /// The direction of the effect's movement, represented as an angle in radians.
    pub direction: f32,
    /// The timestamp indicating when the effect was started.
    pub started_at: Instant,
    /// A flag indicating whether the effect has been spawned.
    pub spawned: bool,
}

impl Effect {
    /// Adjusts the direction of the effect when it collides with the boundaries of the terrain.
    ///
    /// # Parameters
    /// - `terrain_width`: The width of the terrain.
    /// - `terrain_height`: The height of the terrain.
    ///
    /// If the effect's position exceeds the boundaries of the terrain, its direction
    /// is updated to simulate a bounce effect.
    pub fn bounce(&mut self, terrain_width: f32, terrain_height: f32) {
        if self.position.0 <= 0.0 || self.position.0 >= terrain_width {
            self.direction = std::f32::consts::PI - self.direction;
        }
        if self.position.1 <= 0.0 || self.position.1 >= terrain_height {
            self.direction = -self.direction;
        }
    }
}