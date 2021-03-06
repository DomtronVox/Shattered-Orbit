use super::{UIState, StateEvent, UIStateMachine, StationViewState, OrbitalViewState};
use crate::Simulation;


use macroquad::{
    math::vec2,
    window::{screen_width, screen_height},
    ui::{hash, widgets, root_ui},
};


use crate::ecs::create_test_entities;

pub struct MainMenuState {}

impl MainMenuState {
    pub fn new() -> MainMenuState {
        MainMenuState {}
    }
}

impl UIState for MainMenuState {

    fn world_render(&mut self, state_machine: &mut UIStateMachine, sim: &mut Simulation) {
        //no world rendering needed. Might do something later for main menu background.
    }

    fn ui_render(&mut self, state_machine: &mut UIStateMachine, sim: &mut Simulation) {
        let main_menu_id = hash!();
        
        //calculate center position
        let menu_size = vec2(screen_width() / 3., 220.);
        let menu_pos = vec2(
            (screen_width()  / 2.) - (menu_size.x / 2.), 
            (screen_height() /2.) - (menu_size.y / 2.)
        );
        
        //Setup a window that will hold the main menu buttons.
        root_ui().window(main_menu_id, menu_pos, menu_size,         
            |ui| {
                
                //make sure to update window position so it says centered on resize
                ui.move_window(main_menu_id, menu_pos);
                
                widgets::Label::new("Shattered Orbit")
                    .position(vec2(menu_size.x/2. - 60., 10.))
                    .ui(ui);
                
                //setup each of the main menu buttons and handle if their activation
                
                //new game
                if widgets::Button::new("New Game")
                    .size(vec2(menu_size.x / 2., 40.))
                    .position(vec2(menu_size.x/4., 50.))
                    .ui(ui)
                { 
                    create_test_entities(&mut sim.ecs.world);
                     
                    let new_state = OrbitalViewState::new();
                    state_machine.handle_event(
                        StateEvent::ChangeState( Box::new(new_state) )
                    );
                }
                
                //load game
                if widgets::Button::new("Load")
                    .size(vec2(menu_size.x / 2., 40.))
                    .position(vec2(menu_size.x/4., 100.))
                    .ui(ui)
                {  
                    //TODO
                }
                
                //quit game
                if widgets::Button::new("Quit")
                    .size(vec2(menu_size.x / 2., 40.))
                    .position(vec2(menu_size.x/4., 150.))
                    .ui(ui)
                {  
                    state_machine.handle_event(StateEvent::Shutdown); 
                }
            },
        );
    }
}
