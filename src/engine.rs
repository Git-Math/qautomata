use crate::gui;
use crate::universe::types::*;

pub fn run() {
    let mut universe = Universe::new_from_files("./fixtures/state_test_gui.json");
    gui::run(universe);
}
