use crate::creatures::Creature;

pub struct World {
    pub x_len: u32,
    pub y_len: u32,
    pub creatures: Vec<Creature>,
}

impl World {
    pub fn new(x_max: u32, y_max: u32, creature_count: u32) -> World {
        assert!(
            x_max * y_max > creature_count,
            "There must be enough space for all creatures"
        );
        let mut creatures: Vec<Creature> = vec![];
        for i in 0..creature_count {
            let mut generating = true;
            while generating {
                let creature = Creature::new(x_max, y_max);

                for already_placed_creature in &creatures {
                    if already_placed_creature.position == creature.position {
                        break;
                    }
                }
                creatures.push(creature);
                generating = false;
            }
        }

        return World {
            x_len: x_max,
            y_len: y_max,
            creatures,
        };
    }
}

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "World size {}X{}\n", self.x_len, self.y_len).unwrap();
        for creature in &self.creatures {
            write!(
                f,
                "Creature ID {}, with Genome {}, at {}X{}\n",
                creature.id, creature.genome.genes, creature.position.x, creature.position.y
            )
            .unwrap();
        }
        return Ok(());
    }
}
