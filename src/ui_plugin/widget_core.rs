//Copied from https://github.com/bevyengine/bevy/discussions/5522
//Bevy in production: making reusable widgets with bevy and egui #5522
// posted by "aevyrie" of Foresight Mining Software Corporation

use std::collections::HashMap;

use std::hash::Hasher;
use fxhash::FxHasher32;

use bevy::{
    log::debug,
    ecs::{
        system::{SystemState, SystemParam, Resource},
        world::{Mut, World},
    },
};

use bevy_egui::egui::Ui;


//Used in IDing widget to be instanced and nested correctly
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] 
pub struct WidgetId(pub u64);

impl WidgetId { 
     pub fn new(name: &str) -> Self { 
         let bytes = name.as_bytes(); 
         let mut hasher = FxHasher32::default(); 
         hasher.write(bytes); 
         WidgetId(hasher.finish()) 
     }
     
     pub fn with(&self, name: &str) -> Self { 
         Self::new(&format!("{}{name}", self.0)) 
     } 
}


/// A UI widget may have multiple instances. We need to ensure the local state 
/// of these instances is not shared. This hashmap allows us to dynamically 
//  store instance states.
#[derive(Default, Resource)]
struct StateInstances<T: WidgetSystem + 'static > {
    instances: HashMap<WidgetId, SystemState<T>>,
}


//Trait all Widgets should implement
pub trait WidgetSystem: SystemParam {
    fn system(world: &mut World, state: &mut SystemState<Self>, ui: &mut Ui, id: WidgetId);
}


//This function displays the indicated widget, and allows widgets to be nested.
pub fn widget<S: 'static + WidgetSystem>(world: &mut World, ui: &mut Ui, id: WidgetId) {

    // We need to cache `SystemState` to allow for a system's locally tracked state
    if !world.contains_resource::<StateInstances<S>>() {
        // Note, this message should only appear once! If you see it twice in the logs, 
        // the function may have been called recursively, and will panic.
        debug!("Init system state {}", std::any::type_name::<S>());
        world.insert_resource(StateInstances::<S> {
            instances: HashMap::new(),
        });
    }

    world.resource_scope(|world, mut states: Mut<StateInstances<S>>| {

        if !states.instances.contains_key(&id) {
            debug!(
                "Registering system state for widget {id:?} of type {}",
                std::any::type_name::<S>()
            );
            states.instances.insert(id, SystemState::new(world));
        }

        let cached_state = states.instances.get_mut(&id).unwrap();
        S::system(world, cached_state, ui, id);
        cached_state.apply(world);

    });
}




