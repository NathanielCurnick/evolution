pub const NUM_MOTORS: usize = 4;

#[derive(Clone, Debug)]
pub enum Motor {
    Up,    // Moves the creature up
    Down,  // Moves the creature down
    Left,  // Moves the creature left
    Right, // Moves the creature right
}

impl Motor {
    pub fn new(num: u8) -> Motor {
        return match num {
            0 => Self::Up,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Right,
            _ => panic!("Not implemented this motor ID yet {}", num),
        };
    }
}
