extern crate rand;
use rand::thread_rng;
use rand::Rng;
use uuid::Uuid;
#[derive(Clone)]
pub struct Creature {
    pub id: String,
    pub position: Position,
    pub genome: Genome,
}

impl Creature {
    pub fn new(x_max: u32, y_max: u32) -> Creature {
        return Creature {
            id: Uuid::new_v4().to_string(),
            position: Position::gen_new(x_max, y_max),
            genome: Genome::new_random(),
        };
    }
}

#[derive(PartialEq, Copy, Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn gen_new(x_max: u32, y_max: u32) -> Position {
        let mut rng = thread_rng();

        return Position {
            x: rng.gen_range(0..x_max),
            y: rng.gen_range(0..y_max),
        };
    }
}

#[derive(PartialEq, Clone)]
pub struct Genome {
    pub genes: String,
}

impl Genome {
    pub fn new_random() -> Genome {
        // 4294967295 is largest gene size
        let mut rng = thread_rng();

        let x: u32 = rng.gen_range(0..4294967295);
        let hex = format!("{x:X}");

        return Genome { genes: hex };
    }
}

struct Brain {}
