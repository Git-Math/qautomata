mod grid;

use bevy::prelude::*;

use grid::GridPlugin;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GridPlugin)
        //.insert_resource(WindowDescriptor {
        //    width: 1024.0,
        //    height: 1024.0,
        //    title: String::from("Qautomata"),
        //    ..Default::default()
        //})
        .run();
}
