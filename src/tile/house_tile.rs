use crate::{
    entity::{Entity, EntityCast},
    position::Position,
};

use super::{Tile, TileCast};

/// The `HouseTile` struct represents a tile in the game that has a specific position.
pub struct HouseTile {
    /// The position of the house tile in the world.
    position: Position,
}

impl HouseTile {
    /// Creates a new `HouseTile` at the specified position.
    pub fn new(position: &Position) -> Self {
        Self {
            position: position.clone(),
        }
    }
}

impl Entity for HouseTile {
    /// Returns the position of the house tile.
    fn position(&self) -> Option<&Position> {
        Some(&self.position)
    }

    /// Returns a mutable reference to the position of the house tile.
    fn position_mut(&mut self) -> Option<&mut Position> {
        Some(&mut self.position)
    }
}

impl EntityCast for HouseTile {
    /// Attempts to cast the house tile as a generic tile.
    fn as_tile(&self) -> Option<&dyn Tile> {
        Some(self)
    }

    /// Attempts to cast the house tile as a mutable reference to a generic tile.
    fn as_tile_mut(&mut self) -> Option<&mut dyn Tile> {
        Some(self)
    }
}

impl Tile for HouseTile {}

impl TileCast for HouseTile {
    /// Attempts to cast the house tile as a `HouseTile`.
    fn as_house_tile(&self) -> Option<&HouseTile> {
        Some(self)
    }

    /// Attempts to cast the house tile as a mutable reference to a `HouseTile`.
    fn as_house_tile_mut(&mut self) -> Option<&mut HouseTile> {
        Some(self)
    }
}
