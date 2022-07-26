use crate::gui;
use crate::universe::types::*;

pub fn run() {
    let mut universe = Universe::new_from_files("./fixtures/state1.json");
    gui::run(universe);
}
