use bevy::prelude::*;

use bevy::render::mesh::shape::UVSphere;

use crate::{
    general_events::StartNewGame,
    orbital_plugin::Orbit,
};


pub struct ProcGenPlugin;

impl Plugin for ProcGenPlugin {
    fn build(&self, app: &mut App) {
    
        app.add_systems(Update, handle_generation);
    
    }
}


//runs procedural generation requests sent via the various events
fn handle_generation(
  mut commands: Commands, mut ev_newgame: EventReader<StartNewGame>,
  mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>,
  ) {

    for ev in ev_newgame.iter() {
        
        //create camera
        let mut cam = commands.spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        });
        
        //create light
        commands.spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        });
        
        //create earth
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 1.0, sectors: 16, stacks: 16 })),
            material: materials.add(Color::rgb_u8(124, 144, 255).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        });
        
        //create luna
        commands.spawn((PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 0.5, sectors: 16, stacks: 16 })),
            material: materials.add(Color::rgb_u8(255, 255, 255).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
            },
            Orbit::new(5.9722e24, 384748000., 0.0549006, 0.02693043, 0., 0.),
        ));
    }
}
