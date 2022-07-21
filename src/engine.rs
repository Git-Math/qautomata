use crate::universe::types::*;

pub fn run() {
    let mut universe = Universe::new();

    // The counter `n` is temporary until we implement
    // a cleaner way to stop the loop
    for _n in 0..10 {
        universe.step();
    }
}
