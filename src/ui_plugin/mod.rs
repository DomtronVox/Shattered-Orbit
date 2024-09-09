mod widget_core;
pub use widget_core::WidgetId;

mod views;
use views::{main_menu_view, game_setup_view, orbital_view};

pub mod cameras;
use cameras::{orbit_camera_system, OrbitCameraState};


use bevy::{
    prelude::*,
    window::PrimaryWindow,
};
use bevy_egui::{EguiPlugin, EguiContext};


//The current view we are showing
#[derive(Resource)]
enum SelectedView{
    MainMenuView,
    GameSetupView,
    OrbitalView,
}


//Bevy plugin that will setup everything needed for our UI system
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
    
        //We need a resource that tracks what View we currently are in.
        //Defaults to MainMenu view at startup
        app.insert_resource(SelectedView::MainMenuView);
        
        app.add_plugins(EguiPlugin);
        app.add_systems(Update, (
            orbit_camera_system
                .run_if(any_with_component::<OrbitCameraState>),
            draw_ui,
        ));
    }
}



fn draw_ui(world: &mut World) {

    //get egui context
    let mut state = world.query_filtered::<Entity, (With<EguiContext>, With<PrimaryWindow>)>();
    let entity = state.single(&world);
    let mut egui_context = world.get_mut::<EguiContext>(entity).unwrap();
    let ctx = egui_context.get_mut().clone();

    match world.resource::<SelectedView>() {
        SelectedView::MainMenuView => main_menu_view(WidgetId::new("MainMenu"), world, ctx),
        SelectedView::GameSetupView => game_setup_view(WidgetId::new("GameSetup"), world, ctx),
        SelectedView::OrbitalView => orbital_view(WidgetId::new("Orbit"), world, ctx),
    };
}





