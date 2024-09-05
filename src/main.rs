use bevy::prelude::*;

mod sim_time;
use sim_time::{SimTime, advance_sim_time};

mod orbital_plugin;
use orbital_plugin::OrbitalPlugin;

mod ui_plugin;
use ui_plugin::UIPlugin;

mod procgen_plugin;
use procgen_plugin::ProcGenPlugin;

mod gameworld_settings;
mod general_events;
use general_events::StartNewGame;


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0., 0., 0.)))
        
        //Add in the simulation speed clock
        .init_resource::<Time<SimTime>>()
        .add_systems(Update, advance_sim_time)
        
        //add in plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(OrbitalPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(ProcGenPlugin)
        //.add_systems(Startup, setup)
       

        //Add in general events
        .add_event::<StartNewGame>()
        
        
        .run();
}


/*fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

}*/



