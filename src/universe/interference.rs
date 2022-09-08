use num::Zero;
use std::collections::hash_map::DefaultHasher;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use super::types::*;

impl Universe {
    pub fn solve_interference(&mut self) {
        let mut configurations_hash: HashMap<u64, usize> = HashMap::new();

        for i in 0..self.state.len() {
            let mut sorted_living_cells = self.state[i]
                .living_cells
                .keys()
                .cloned()
                .collect::<Vec<Coordinates>>();
            sorted_living_cells.sort_unstable();

            let mut hasher = DefaultHasher::new();
            sorted_living_cells.hash(&mut hasher);
            let configuration_hash = hasher.finish();

            match configurations_hash.entry(configuration_hash) {
                Vacant(entry) => {
                    entry.insert(i);
                }
                Occupied(entry) => {
                    let configuration_i = *entry.get();
                    let current_amplitude = self.state[i].amplitude;
                    let current_norm = current_amplitude.norm_sqr();
                    let interference_amplitude = self.state[configuration_i].amplitude;
                    let interference_norm = interference_amplitude.norm_sqr();
                    let sum_amplitude = current_amplitude + interference_amplitude;
                    let sum_amplitude_norm = sum_amplitude.norm_sqr();
                    let norm_delta = sum_amplitude_norm - current_norm - interference_norm;

                    self.state[configuration_i].amplitude += current_amplitude;
                    self.state[i].amplitude.set_zero();

                    for coordinates in self.state[i].living_cells.keys() {
                        *self.combined_state.get_mut(coordinates).unwrap() += norm_delta;
                    }

                    println!("Interference");
                }
            }
        }

        self.state.retain(|configuration| {
            configuration.amplitude.re > 0.001
                || configuration.amplitude.re < -0.001
                || configuration.amplitude.im > 0.001
                || configuration.amplitude.im < -0.001
        });
        self.combined_state
            .retain(|_, norm_sum| *norm_sum > 0.00001); // Care maybe performance issues
    }
}
