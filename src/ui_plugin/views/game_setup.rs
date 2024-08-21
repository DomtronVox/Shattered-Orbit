

use bevy::prelude::*;
use bevy::ecs::system::SystemState;


use bevy_egui::egui::{self};

use super::super::{
    widget_core::WidgetId,
    SelectedView,
};

use crate::{
    default,
    general_events::StartNewGame,
    gameworld_settings::GameWorldSettings,
};



//The game setup view handles configuring and eventually firing off everything that starts a new game.
pub fn game_setup_view (_id: WidgetId, world: &mut World, ctx: egui::Context ) {

     egui::Window::new("Game Setup")
        .collapsible(false)
        .title_bar(false)
        .fixed_pos((200.,200.))
        .fixed_size((1000., 400.))
        .show(&ctx, |ui| {
           
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.label("Hello World!");
                });
           
        
           
            //Confirmation/reset/exit Buttons along the bottom
            egui::Grid::new("Control Buttons")
                .num_columns(3)
                .show(ui, |ui| {
               
                    if ui.button("Cancel").clicked() {
                        world.insert_resource::<SelectedView>(SelectedView::MainMenuView);
                    }
                    
                    ui.add_space(100.0);
                    
                    if ui.button("Start game").clicked() {

                        //TODO collect game settings from UI elements and populate GameWorldSettings with them
                        
                        //Get needed variables for bundle creation
                        let mut system_state: SystemState<(
                            EventWriter<StartNewGame>
                        )> = SystemState::new(world);
                        
                        let mut ev_newgame = system_state.get_mut(world);
                        ev_newgame.send(StartNewGame(GameWorldSettings));
                        
                        
                        system_state.apply(world);
                        
                        //Shift view to in game orbital view
                        world.insert_resource::<SelectedView>(SelectedView::OrbitalView);
                    }
                    
                    ui.end_row();
               
            });
           
            
        });

} 

