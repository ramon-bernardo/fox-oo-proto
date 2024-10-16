use std::sync::Arc;

use entity::*;
use position::*;
use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Diagnostics, Vm};
use tile::house_tile::*;
use tile::*;

mod entity;
mod position;
mod tile;

fn main() -> anyhow::Result<()> {
    let mut context = rune_modules::default_context()?;
    context.install(house_tile_module()?)?;
    context.install(position_module()?)?;

    let mut sources = rune::sources! {
        entry => {
            pub fn main(tile) {
                dbg!(tile.position.x);
                tile.position.x = 2;
                dbg!(tile.position.x);
            }
        }
    };

    let mut diagnostics = Diagnostics::new();

    let result = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build();

    if !diagnostics.is_empty() {
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        diagnostics.emit(&mut writer, &sources)?;
    }

    let runtime = context.runtime()?;
    let unit = result?;

    let runtime = Arc::new(runtime);
    let mut vm = Vm::new(runtime, Arc::new(unit));

    // Create a new position.
    let initial_position = Position { x: 10, y: 20, z: 5 };
    // Create a new HouseTile with the initial position.
    let mut house_tile = HouseTile::new(&initial_position);
    vm.call(["main"], (&mut house_tile,))?;
    Ok(())
}

fn main2() {
    // Create a new position.
    let initial_position = Position { x: 10, y: 20, z: 5 };

    // Create a new HouseTile with the initial position.
    let mut house_tile = HouseTile::new(&initial_position);

    // Print the initial position of the HouseTile.
    if let Some(position) = house_tile.position() {
        println!(
            "Initial Position - x: {}, y: {}, z: {}",
            position.x, position.y, position.z
        );
    } else {
        println!("Failed to get the initial position.");
    }

    // Modify the HouseTile position.
    if let Some(position_mut) = house_tile.position_mut() {
        position_mut.x = 15;
        position_mut.y = 25;
        position_mut.z = 10;
    }

    // Print the modified position of the HouseTile.
    if let Some(position) = house_tile.position() {
        println!(
            "Modified Position - x: {}, y: {}, z: {}",
            position.x, position.y, position.z
        );
    } else {
        println!("Failed to get the modified position.");
    }

    // Casting to the Tile trait.
    if let Some(tile) = house_tile.as_tile() {
        println!("HouseTile successfully cast to Tile.");
    } else {
        println!("Failed to cast HouseTile to Tile trait.");
    }

    // Casting back to HouseTile.
    if let Some(cast_house_tile) = house_tile.as_house_tile() {
        println!(
            "HouseTile successfully cast back to HouseTile with position - x: {}, y: {}, z: {}",
            cast_house_tile.position().unwrap().x,
            cast_house_tile.position().unwrap().y,
            cast_house_tile.position().unwrap().z
        );
    } else {
        println!("Failed to cast back to HouseTile.");
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_house_tile_creation() {
        let position = Position { x: 10, y: 20, z: 5 };
        let house_tile = HouseTile::new(&position);
        assert_eq!(house_tile.position().unwrap().x, 10);
        assert_eq!(house_tile.position().unwrap().y, 20);
        assert_eq!(house_tile.position().unwrap().z, 5);
    }

    #[test]
    fn test_position_mutability() {
        let mut house_tile = HouseTile::new(&Position { x: 10, y: 20, z: 5 });
        let position_mut = house_tile.position_mut().unwrap();
        position_mut.x = 15;
        position_mut.y = 25;
        assert_eq!(house_tile.position().unwrap().x, 15);
        assert_eq!(house_tile.position().unwrap().y, 25);
    }

    #[test]
    fn test_as_tile() {
        let house_tile = HouseTile::new(&Position { x: 1, y: 1, z: 1 });
        let tile = house_tile.as_tile();
        assert!(tile.is_some());
    }

    #[test]
    fn test_as_tile_mut() {
        let mut house_tile = HouseTile::new(&Position { x: 1, y: 1, z: 1 });
        let tile = house_tile.as_tile_mut();
        assert!(tile.is_some());
    }

    #[test]
    fn test_as_house_tile() {
        let house_tile = HouseTile::new(&Position { x: 5, y: 5, z: 0 });
        let cast_tile = house_tile.as_house_tile();
        assert!(cast_tile.is_some());
        assert_eq!(cast_tile.unwrap().position().unwrap().x, 5);
    }

    #[test]
    fn test_as_house_tile_mut() {
        let mut house_tile = HouseTile::new(&Position { x: 5, y: 5, z: 0 });
        let cast_tile_mut = house_tile.as_house_tile_mut();
        let mut_tile = cast_tile_mut.unwrap();
        mut_tile.position_mut().unwrap().x = 10;
        assert_eq!(house_tile.position().unwrap().x, 10);
    }
}
