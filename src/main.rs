use bevy::prelude::*;



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
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        
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


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    
}


