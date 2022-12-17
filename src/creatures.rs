extern crate rand;
use std::any::Any;
use std::collections::HashMap;
use std::fmt;

use rand::thread_rng;
use rand::Rng;
use uuid::Uuid;

use crate::sensors::Sensor;

#[derive(Clone)]
pub struct Creature {
    pub id: String,
    pub position: Position,
    pub genome: Genome,
    pub brain: Brain,
}

impl Creature {
    pub fn new(x_max: usize, y_max: usize, num_genes: usize) -> Creature {
        let id = Uuid::new_v4().to_string();
        let position = Position::gen_new(x_max, y_max);
        let genome = Genome::new(num_genes);
        let brain = Brain::new(&genome);
        return Creature {
            id: id,
            position: position,
            genome: genome,
            brain: brain,
        };
    }

    pub fn calculate_activity(&self) -> Option<Motor> {
        for synapse in &self.brain.connections {}
        todo!();
    }
}

#[derive(PartialEq, Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn gen_new(x_max: usize, y_max: usize) -> Position {
        let mut rng = thread_rng();
        return Position {
            x: rng.gen_range(0..x_max),
            y: rng.gen_range(0..y_max),
        };
    }
}

#[derive(Clone)]
pub struct Genome {
    pub genome: Vec<Gene>,
}

impl Genome {
    pub fn new(num: usize) -> Genome {
        let mut genes = vec![];

        for _ in 0..num {
            genes.push(Gene::new_random());
        }

        return Genome { genome: genes };
    }
}

impl fmt::Display for Genome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = "".to_string();
        for gene in &self.genome {
            s.push_str(&gene.as_hex());
            s.push('-');
        }

        s.pop();

        write!(f, "{}", s)
    }
}

#[derive(PartialEq, Clone)]
pub struct Gene {
    pub source: u8,
    pub source_id: u8,
    pub sink: u8,
    pub sink_id: u8,
    pub weight: u16,
}

impl fmt::Display for Gene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({:b}-{:b}-{:b}-{:b}-{:b})",
            self.source, self.source_id, self.sink, self.sink_id, self.weight
        )
    }
}

impl Gene {
    pub fn new_random() -> Gene {
        // 4294967295 is largest gene size
        // This is 2^32
        // So, we basically generate a gene which can encode a neuron
        // Convert this to a 32 bit binary
        // Digits
        // 0 - Source type. 1 is a sensory neuron, 0 is an internal neuron. 1 Bit
        // 1-7 - source ID. 7 Bit
        // 8 - Sink type. 1 is a sensory neuron, 0 is an internal neuron. 1 Bit
        // 9-15 - Sink ID. 7 Bit
        // 16-31 - Weight. 16 Bit

        let mut rng = thread_rng();

        let source = rng.gen_range(0..=1);
        let source_id = rng.gen_range(0..=3);
        let sink = rng.gen_range(0..=1);
        let sink_id = rng.gen_range(0..=3);
        let weight = rng.gen_range(0..=65535);

        // let x: u32 = rng.gen_range(0..4294967295);
        // let hex = format!("{x:X}");
        return Gene {
            source,
            source_id,
            sink,
            sink_id,
            weight,
        };
    }

    pub fn as_hex(&self) -> String {
        let bsource = format!("{:01b}", self.source);
        let bsource_id = format!("{:07b}", self.source_id);
        let bsink = format!("{:01b}", self.sink);
        let bsink_id = format!("{:07b}", self.sink_id);
        let bweight = format!("{:016b}", self.weight);

        let binary_string = format!("{}{}{}{}{}", bsource, bsource_id, bsink, bsink_id, bweight);
        let bin = usize::from_str_radix(&binary_string, 2).unwrap();

        let hex = format!("{bin:X}");

        return hex;
    }
}

#[derive(Clone)]
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
// For now there is only one kind of internal neuron
// I leave this in case there could be more later
#[derive(Clone, PartialEq)]
pub enum Internal {
    Internal,
}

#[derive(Clone, PartialEq)]
pub enum InputNeuron {
    Sensor(Sensor),
    Internal(Internal),
}

#[derive(Clone)]
pub enum OutputNeuron {
    Internal(Internal),
    Motor(Motor),
}

#[derive(Clone)]
pub struct Synapse {
    pub from: InputNeuron,
    pub to: OutputNeuron,
    pub weight: f64,
}

impl Synapse {
    pub fn new(gene: &Gene) -> Synapse {
        let source = if gene.source == 0 {
            // Source of 0 means sensor neuron
            let sensor_type = Sensor::new(gene.source_id);
            InputNeuron::Sensor(sensor_type)
        } else if gene.source == 1 {
            // Source of 1 means internal neuron
            InputNeuron::Internal(Internal::Internal)
        } else {
            panic!("This is not a valid source ID: {}. Acceptable values are 0 (sensor) or 1 (internal neuron)", gene.source)
        };

        let sink = if gene.sink == 0 {
            let motor_type = Motor::new(gene.sink_id);
            OutputNeuron::Motor(motor_type)
        } else if gene.sink == 1 {
            OutputNeuron::Internal(Internal::Internal)
        } else {
            panic!("This is not a valid sink ID: {}. Acceptable values are 0 (motor) or 1 (internal neuron)", gene.sink)
        };

        // Want the final weight to be a floating point value - this is easier
        let gene_weight = gene.weight as f64;
        let weight = (gene_weight - gene_weight / 2.0) / 10_000_f64;

        return Synapse {
            from: source,
            to: sink,
            weight: weight,
        };
    }

    pub fn calculate_activation(&self) -> f64 {}
}

#[derive(Clone)]
pub struct Brain {
    pub from_sensor: Vec<Synapse>,
    pub from_internal: Vec<Synapse>,
}

impl Brain {
    pub fn new(genome: &Genome) -> Brain {
        let mut from_sensor = vec![];
        let mut from_internal = vec![];
        for gene in &genome.genome {
            let synapse = Synapse::new(gene);
            if synapse.from == InputNeuron::Internal(Internal::Internal) {
                from_internal.push(synapse);
            } else {
                from_sensor.push(synapse);
            }
        }

        return Brain {
            from_sensor,
            from_internal,
        };
    }

    pub fn calculate_activity(&self) -> Motor {
        let mut internal_hm: HashMap<u8, f64> = HashMap::new();
        let mut motor_hm: HashMap<u8, f64> = HashMap::new();
        for synapse in &self.from_sensor {}
    }
}
