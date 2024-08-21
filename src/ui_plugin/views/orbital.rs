

use bevy::ecs::world::World;

use bevy_egui::egui;

use super::super::widget_core::WidgetId;

//The orbit view allows viewing, navigating and interacting with the orbital environment.
pub fn orbital_view (_id: WidgetId, _world: &mut World, ctx: egui::Context ) {

     //Left window element selector
     egui::Window::new("Seletor")
        .collapsible(false)
        .title_bar(false)
        .fixed_pos((10.,10.))
        .fixed_size((100., 300.))
        .show(&ctx, |ui| {
        
     });

     //Right panel, Time control, selection status, Notification/message area
     
     //bottom panel, view selection buttons (orbit, station, planet)

} 
