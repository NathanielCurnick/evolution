use crate::world::World;

pub mod creatures;
pub mod world;

#[cfg(test)]
mod tests;

fn main() {
    println!("Hello, world!");

    let world = World::new(128, 128, 10);

    println!("{}", world);
}
