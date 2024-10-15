use house_tile::HouseTile;

use crate::entity::Entity;

pub mod house_tile;

/// The `Tile` trait represents an entity that behaves as a tile within the system.
pub trait Tile: TileCast + Entity {}

/// The `TileCast` trait provides methods for casting a tile entity into specific tile types.
/// This allows for dynamic type checking and behavior specific to certain types of tiles.
pub trait TileCast {
    /// Attempts to cast the tile as a `HouseTile`.
    fn as_house_tile(&self) -> Option<&HouseTile> {
        None
    }

    /// Attempts to cast the tile as a mutable reference to a `HouseTile`.
    fn as_house_tile_mut(&mut self) -> Option<&mut HouseTile> {
        None
    }
}
