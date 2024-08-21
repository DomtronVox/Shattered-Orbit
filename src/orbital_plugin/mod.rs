use bevy::prelude::*;

mod orbit;
pub use orbit::{orbit_motion_system, Orbit};

pub struct OrbitalPlugin;

impl Plugin for OrbitalPlugin {
    fn build(&self, app: &mut App) {
    
        app.add_systems(Update, orbit_motion_system);
    
    }
}
