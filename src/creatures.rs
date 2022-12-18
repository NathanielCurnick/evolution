use peroxide::prelude::*;

extern crate rand;
use std::any::Any;
use std::collections::HashMap;
use std::fmt;

use peroxide::matrix;
use peroxide::prelude::Matrix;
use rand::thread_rng;
use rand::Rng;
use uuid::Uuid;

use crate::motors::Motor;
use crate::motors::NUM_MOTORS;
use crate::sensors::Sensor;
use crate::sensors::NUM_SENSORS;
use crate::world::World;

#[derive(Clone)]
pub struct Creature {
    pub id: String,
    pub genome: Genome,
    pub brain: Brain,
}

impl Creature {
    pub fn new(
        x_max: usize,
        y_max: usize,
        num_genes: usize,
        num_internal: u8,
    ) -> (Creature, Position) {
        let id = Uuid::new_v4().to_string();
        let position = Position::gen_new(x_max, y_max);
        let genome = Genome::new(num_genes, num_internal);
        let brain = Brain::new(&genome, num_internal as usize);
        return (
            Creature {
                id: id,
                genome: genome,
                brain: brain,
            },
            position,
        );
    }

    pub fn calculate_activity(&self, world: &World, position: Position) -> Motor {
        return self.brain.calculate_activity(world, position);
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
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
    pub fn new(num: usize, num_internal: u8) -> Genome {
        let mut genes = vec![];

        for _ in 0..num {
            genes.push(Gene::new_random(num_internal));
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
    pub fn new_random(num_internal: u8) -> Gene {
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
        let source_id = if source == 0 {
            // This means the source is a sensor neuron
            rng.gen_range(0..NUM_SENSORS as u8)
        } else if source == 1 {
            // This means the source is an internal neuron
            rng.gen_range(0..num_internal as u8)
        } else {
            panic!("Generation outside of range");
        };

        let sink = if source == 0 {
            0
        } else if source == 1 {
            1
        } else {
            panic!("Generated outside of range");
        };
        let sink_id = if sink == 0 {
            // This means the sink is an internal neuron
            rng.gen_range(0..num_internal as u8)
        } else if sink == 1 {
            // Thi means the sink is a motor neuron
            rng.gen_range(0..NUM_MOTORS as u8)
        } else {
            panic!("Generated outside of range");
        };

        rng.gen_range(0..=3);
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
pub struct Brain {
    // Legacy above, might be useful
    pub weights_sensor: Matrix,
    pub weights_internal: Matrix,
    pub sensors: Vec<Sensor>,
}

impl Brain {
    pub fn new(genome: &Genome, num_internal: usize) -> Brain {
        // For weights sensor, r is the number of internal/output neurons
        // c is the number of possible sensors
        // TODO: Need to be able to vary these in some particular way
        let mut weights_sensor = zeros(num_internal, NUM_SENSORS);

        // For internal weights i.e. weights from internal neurons to motor neurons
        // r is the number of motor neurons/output
        // c is the number of internal neurons
        let mut weights_internal = zeros(NUM_MOTORS, num_internal);

        let mut sensors: Vec<Sensor> = vec![];
        for gene in &genome.genome {
            if gene.source == 0 {
                let weight_g = gene.weight as f64;

                // TODO: is this giving sensible weights?
                let weight = (weight_g - weight_g / 2.0) / 10_000_f64;

                sensors.push(Sensor::new(gene.source_id));

                // TODO: is this right?
                weights_sensor[(gene.sink_id as usize, gene.source_id as usize)] = weight;
            } else if gene.source == 1 {
                let weight_g = gene.weight as f64;

                // TODO: is this giving sensible weights?
                let weight = (weight_g - weight_g / 2.0) / 10_000_f64;

                // TODO: is this right?
                weights_internal[(gene.sink_id as usize, gene.source_id as usize)] = weight;
            } else {
                panic!("Generated index out of range");
            }
        }

        return Brain {
            weights_sensor,
            weights_internal,
            sensors,
        };
    }

    pub fn calculate_activity(&self, world: &World, position: Position) -> Motor {
        let mut input_vec: Vec<f64> = vec![0.0; NUM_SENSORS]; // TODO: Need to abstract this to take any number of elements
        for sensor in &self.sensors {
            let fun = sensor.get_fun();
            let activation = fun(world, position);
            let pos = sensor.get_id();
            input_vec[pos] = activation;
        }
        println!("The sensor matrix looks like {:?}", input_vec);
        let sensor_matrix = matrix(input_vec, 4, 1, Row);

        let internal_neuron_activation = &self.weights_sensor * &sensor_matrix;

        let output_neuron_activation = &self.weights_internal * &internal_neuron_activation;

        let output_vec = output_neuron_activation.data;

        let index = output_vec
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(index, _)| index)
            .unwrap();

        return Motor::new(index as u8);
    }
}
