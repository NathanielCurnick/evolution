use crate::world::World;

pub mod creatures;
pub mod sensors;
pub mod world;

#[cfg(test)]
mod tests;

fn main() {
    println!("Hello, world!");
    // Initial world generation
    let world = World::new(128, 128, 10);

    // One tick
}

fn tick(world: &World) -> World {
    let mut new_world = World::blank(world.x_len, world.y_len);

    for row in &world.locs {
        for tile in row {
            if tile.is_some() {
                let creature = tile.as_ref().unwrap();
            }
        }
    }

    todo!();
}
