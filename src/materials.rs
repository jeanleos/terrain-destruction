
// -----------------------------------------------------------------------------
// File: materials.rs
// Description: Contains the Material enum used to represent different types of terrain materials.
// Author(s): DIARRA Amara & SERRANO Jean-Léo
// License: CC BY-NC 4.0
// Created: March 15, 2025
// Last modified: March 25, 2025
// Version: 1.0
// -----------------------------------------------------------------------------

/// Represents different types of terrain materials.
/// 
/// # Variants
/// 
/// - `Air`: Represents empty space or air.
/// - `Grass`: Represents grassy terrain.
/// - `Rock`: Represents rocky terrain.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Material {
    Air,
    Grass,
    Rock,
}
