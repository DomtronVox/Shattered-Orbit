use bevy::ecs::event::Event;


//Event to initiate a new game
use crate::gameworld_settings::GameWorldSettings;

#[derive(Event)]
pub struct StartNewGame(pub GameWorldSettings);
