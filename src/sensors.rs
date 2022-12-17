use crate::{creatures::Position, world::World};

#[derive(Clone, PartialEq)]
pub enum Sensor {
    Up,    // Detects another creature in the up direction
    Down,  // Detects another creature in the down direction
    Left,  // Detects another creature in the left direction
    Right, // Detects another creature in the right direction
}

impl Sensor {
    pub fn new(num: u8) -> Sensor {
        return match num {
            0 => Self::Up,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Right,
            _ => panic!("Not implemented this sensor ID yet {}", num),
        };
    }
}

// TODO: I think these functions can be abstracted into a single more general function
pub fn up_activation(world: &World, pos: Position) -> f64 {
    if pos.y == world.y_len {
        return 0.0;
    }

    return if world.locs[pos.x][pos.y + 1].is_none() {
        0.0
    } else {
        1.0
    };
}

pub fn down_activation(world: &World, pos: Position) -> f64 {
    if pos.y == 0 {
        return 0.0;
    }

    return if world.locs[pos.x][pos.y - 1].is_none() {
        0.0
    } else {
        1.0
    };
}

pub fn left_activation(world: &World, pos: Position) -> f64 {
    if pos.x == 0 {
        return 0.0;
    }

    return if world.locs[pos.x - 1][pos.y].is_none() {
        0.0
    } else {
        1.0
    };
}

pub fn right_activation(world: &World, pos: Position) -> f64 {
    if pos.x == world.x_len {
        return 0.0;
    }

    return if world.locs[pos.x + 1][pos.y].is_none() {
        0.0
    } else {
        1.0
    };
}
