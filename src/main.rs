#[macro_use]
extern crate peroxide;

use crate::{creatures::Position, motors::Motor, world::World};

pub mod creatures;
pub mod motors;
pub mod sensors;
pub mod world;

// Tests
#[cfg(test)]
mod tests;

fn main() {
    println!("Hello, world!");
    // Initial world generation
    let mut world = World::new(128, 128, 100);
    println!("{}", world);

    for _ in 0..20 {
        world = tick(&world);
    }
    println!("{}", world);
}

fn tick(world: &World) -> World {
    let mut new_world = World::blank(world.x_len, world.y_len);

    for (i, row) in world.locs.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if tile.is_some() {
                let creature = tile.as_ref().unwrap();
                let pos = Position { x: i, y: j };
                println!("The creature is at {:?}", pos);
                let action = creature.calculate_activity(world, pos.clone());
                println!("This creatre has decided to {:?}", action);
                // Implement moving logic here
                // Can only move if the tile exists + nobody else is there
                // Remember check from the OLD world and then move into the new world

                match action {
                    Motor::Up => {
                        if j + 1 == world.y_len
                            || world.locs[i][j + 1].is_some()
                            || new_world.locs[i][j + 1].is_some()
                        {
                            new_world.locs[i][j] = Some(creature.clone());
                        } else {
                            new_world.locs[i][j + 1] = Some(creature.clone());
                        }
                    }
                    Motor::Down => {
                        if j - 1 == 0
                            || world.locs[i][j - 1].is_some()
                            || new_world.locs[i][j - 1].is_some()
                        {
                            new_world.locs[i][j] = Some(creature.clone());
                        } else {
                            new_world.locs[i][j - 1] = Some(creature.clone());
                        }
                    }
                    Motor::Left => {
                        if i - 1 == 0
                            || world.locs[i - 1][j].is_some()
                            || new_world.locs[i - 1][j].is_some()
                        {
                            new_world.locs[i][j] = Some(creature.clone());
                        } else {
                            new_world.locs[i - 1][j] = Some(creature.clone());
                        }
                    }
                    Motor::Right => {
                        if i + 1 == world.x_len
                            || world.locs[i + 1][j].is_some()
                            || new_world.locs[i + 1][j].is_some()
                        {
                            println!("I could not move");
                            new_world.locs[i][j] = Some(creature.clone());
                        } else {
                            println!("I could move");
                            println!("I am putting the creature into [{}, {}]", i + 1, j);
                            new_world.locs[i + 1][j] = Some(creature.clone());
                        }
                    }
                }
            }
        }
    }

    return new_world;
}
