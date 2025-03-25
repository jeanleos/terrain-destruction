// -----------------------------------------------------------------------------
// File: cell.rs
// Description: A cell structure representing a single unit of terrain.
// Author(s): DIARRA Amara & SERRANO Jean-Léo
// License: CC BY-NC 4.0
// Created: March 15, 2025
// Last modified: March 25, 2025
// Version: 1.0
// -----------------------------------------------------------------------------

use crate::materials::Material;

/// Represents a cell with specific properties such as material and durability.
///
/// # Fields
/// - `material`: The material that the cell is made of.
/// - `durability`: A floating-point value representing the durability of the cell.
#[derive(Debug, Clone)]
pub struct Cell {
    pub material: Material,
    pub durability: f32,
}