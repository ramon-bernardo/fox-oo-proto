use crate::{position::Position, tile::Tile};

/// The `Entity` trait defines common behaviors for all entities.
pub trait Entity: EntityCast {
    /// Returns the position of the entity.
    fn position(&self) -> Option<&Position> {
        if let Some(tile) = self.as_tile() {
            tile.position()
        } else {
            None
        }
    }

    /// Returns a mutable reference to the position.
    fn position_mut(&mut self) -> Option<&mut Position> {
        if let Some(tile) = self.as_tile_mut() {
            tile.position_mut()
        } else {
            None
        }
    }
}

/// The `EntityCast` trait provides methods for casting an entity to various specific types.
pub trait EntityCast {
    /// Attempts to cast the entity as a tile.
    fn as_tile(&self) -> Option<&dyn Tile> {
        None
    }

    /// Attempts to cast the entity as a mutable tile.
    fn as_tile_mut(&mut self) -> Option<&mut dyn Tile> {
        None
    }
}
