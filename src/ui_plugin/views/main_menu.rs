
use bevy::{
    ecs::world::World,
    app::AppExit,
};


use bevy_egui::egui::{self, Color32, Stroke, RichText, Button};

use super::super::{
    widget_core::WidgetId,
    SelectedView,
};

//The main menu view that people first see which lets them manage settings and start games.
pub fn main_menu_view (_id: WidgetId, world: &mut World, ctx: egui::Context ) {

     //Setup Title text
     egui::Window::new("Title Text")
        .collapsible(false)
        .title_bar(false)
        .fixed_pos((400.,100.))
        .resizable(false)
        .frame(egui::Frame::none()
                    .fill(Color32::TRANSPARENT)
                    .stroke(Stroke::NONE)
              )
        .show(&ctx, |ui| {
        
            //Setup Title
            ui.label( RichText::new("Shattered Orbit").size(30.) );
        
        });
    
    
    
    egui::Window::new("Main Menu")
        .collapsible(false)
        .title_bar(false)
        .fixed_pos((100.,250.))
        .fixed_size((300., 800.))
        .show(&ctx, |ui| {
        
        
            if ui.add_sized([100., 40.], Button::new( RichText::new("New").size(20.) )).clicked() {
                world.insert_resource::<SelectedView>(SelectedView::GameSetupView);
            }
            
            ui.add_sized([100., 40.], Button::new( RichText::new("Load").size(20.) ));
            
            ui.add_sized([100., 40.], Button::new( RichText::new("Settings").size(20.) ));
            
            if ui.add_sized([100., 40.], Button::new( RichText::new("Quit").size(20.) )).clicked() {
                world.send_event(AppExit);
            }

            
        });

} 
