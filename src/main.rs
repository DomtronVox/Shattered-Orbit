use bevy::prelude::*;


mod ui_plugin;
use ui_plugin::UIPlugin;

//mod orbital_plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UIPlugin)
        //.add_systems(Startup, setup)
        .run();
}



