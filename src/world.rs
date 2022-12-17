use crate::creatures::Creature;

pub struct World {
    pub x_len: usize,
    pub y_len: usize,
    pub locs: Vec<Vec<Option<Creature>>>,
}

impl World {
    pub fn new(x_max: usize, y_max: usize, creature_count: usize) -> World {
        assert!(
            x_max * y_max > creature_count,
            "There must be enough space for all creatures"
        );

        let mut locs: Vec<Vec<Option<Creature>>> = vec![vec![None; y_max]; x_max];
        for i in 0..creature_count {
            let mut generating = true;
            while generating {
                let creature = Creature::new(x_max, y_max, 3);
                let x = creature.position.x;
                let y = creature.position.y;
                if locs[x][y].is_none() {
                    locs[x][y] = Some(creature);
                    generating = false;
                }
            }
        }

        return World {
            x_len: x_max,
            y_len: y_max,
            locs,
        };
    }

    pub fn blank(x_max: usize, y_max: usize) -> World {
        let locs: Vec<Vec<Option<Creature>>> = vec![vec![None; y_max]; x_max];
        return World {
            x_len: x_max,
            y_len: y_max,
            locs,
        };
    }
}

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "World size {}X{}\n", self.x_len, self.y_len).unwrap();
        for col in &self.locs {
            for creature in col {
                if creature.is_some() {
                    let unwrapped_creature = creature.as_ref().unwrap();
                    write!(
                        f,
                        "Creature ID {}, with Genome {}, at {}X{}\n",
                        unwrapped_creature.id,
                        unwrapped_creature.genome,
                        unwrapped_creature.position.x,
                        unwrapped_creature.position.y
                    )
                    .unwrap();
                }
            }
        }
        return Ok(());
    }
}
