use crate::gui;
use crate::universe::types::*;

pub fn run() {
    let mut universe = Universe::new_from_files("./fixtures/state1.json");

    gui::run();

    // The counter `n` is temporary until we implement
    // a cleaner way to stop the loop
    for _n in 0..100 {
        universe.step();
        /*print!(
            "t: {:?}\nis_even_state: {:?}\nstate: {:?}\n\n",
            _n, universe.is_even_step, universe.state
        );*/
        if universe.state.len() > 1000 {
            universe.measure();
            print!("{:#?}\n", universe.state);
        }
        print!("{}\n", universe.state.len());
    }
}
