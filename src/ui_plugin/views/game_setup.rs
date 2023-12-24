

use bevy::ecs::world::World;


use bevy_egui::egui::{self};

use super::super::{
    widget_core::WidgetId,
    SelectedView,
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
           
        
           
            //Confirmation Buttons along the bottom
            egui::Grid::new("TextLayoutDemo")
                .num_columns(3)
                .show(ui, |ui| {
               
                    if ui.button("Cancel").clicked() {
                        world.insert_resource::<SelectedView>(SelectedView::MainMenuView);
                    }
                    
                    ui.add_space(100.0);
                    
                    if ui.button("Start game").clicked() {

                        //collect game settings data from UI
                        
                        //Fire off processes
                        
                        //Shift view to in game orbital view
                        world.insert_resource::<SelectedView>(SelectedView::OrbitalView);
                    }
                    
                    ui.end_row();
               
            });
           
            
        });

} 
