mod step_calculator;
mod universe_types;

use std::collections::HashMap;
use num::complex::Complex;

use universe_types::Coordinates;
use universe_types::LivingCell;
use universe_types::Configuration;
use universe_types::Universe;

fn main() {

    let coordinates = Coordinates {
        x: 1,
        y: 2,
    };
    let living_cell: LivingCell = HashMap::new();
    let configuration: Configuration = Vec::new();
    let universe = Universe {
        amplitude: Complex::new(10.1, 20.2),
        configuration,
    };
    step_calculator::compute_universe_step(universe);
    println!("Hello, world!");
}
