use rune::{alloc::clone::TryClone, Any, Module};

/// The `Position` struct represents a 2D position in space, with the `z` coordinate representing the floor.
#[derive(Any, TryClone, Clone, Copy, Debug)]
pub struct Position {
    #[rune(get, set, copy)]
    pub x: u16,
    #[rune(get, set, copy)]
    pub y: u16,
    #[rune(get, set, copy)]
    pub z: u8,
}

pub fn position_module() -> anyhow::Result<Module> {
    let mut module = Module::new();
    module.ty::<Position>()?;
    Ok(module)
}
