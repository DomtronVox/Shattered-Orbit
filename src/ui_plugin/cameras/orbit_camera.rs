//Mostly pulled from https://bevy-cheatbook.github.io/cookbook/pan-orbit-camera.html
//  with some modification.

//Camera used in the orbital view, anchors to and moves around selected object

use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;


///Action to do when scrolling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScrollOrbitCameraAction {
    //Pan,
    Orbit,
    Zoom,
}



// Bundle to spawn our custom camera easily
#[derive(Bundle, Default)]
pub struct OrbitCameraBundle {
    pub camera: Camera3dBundle,
    pub state: OrbitCameraState,
    pub settings: OrbitCameraSettings,
}


// The internal state of the pan-orbit controller
#[derive(Component)]
pub struct OrbitCameraState {
    pub center: Vec3,
    pub radius: f32,
    pub upside_down: bool,
    pub pitch: f32,
    pub yaw: f32,
}


impl Default for OrbitCameraState {
    fn default() -> Self {
        Self {
            center: Vec3::ZERO,
            radius: 1.0,
            upside_down: false,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}



/// The configuration of the pan-orbit controller
#[derive(Component)]
pub struct OrbitCameraSettings {
    
    /// World units per pixel of mouse motion
    pub pan_sensitivity: f32,
    /// Radians per pixel of mouse motion
    pub orbit_sensitivity: f32,
    /// Exponent per pixel of mouse motion
    pub zoom_sensitivity: f32,
    /// For devices with a notched scroll wheel, like desktop mice
    pub scroll_line_sensitivity: f32,
    /// For devices with smooth scrolling, like touchpads
    pub scroll_pixel_sensitivity: f32,
    
    
    /// Key to hold for panning
    pub pan_key: Option<KeyCode>,
    /// Key to hold for orbiting
    pub orbit_key: Option<KeyCode>,
    /// Key to hold for zooming
    pub zoom_key: Option<KeyCode>,
    
    /// What action is bound to the scroll wheel?
    pub scroll_action: Option<ScrollOrbitCameraAction>,
        
}

impl Default for OrbitCameraSettings {
    fn default() -> Self {
        Self {
            pan_sensitivity: 0.001, // 1000 pixels per world unit
            orbit_sensitivity: 0.1f32.to_radians(), // 0.1 degree per pixel
            zoom_sensitivity: 0.01,
            pan_key: Some(KeyCode::ControlLeft),
            orbit_key: Some(KeyCode::AltLeft),
            zoom_key: Some(KeyCode::ShiftLeft),
            scroll_action: Some(ScrollOrbitCameraAction::Zoom),
            scroll_line_sensitivity: 16.0, // 1 "line" == 16 "pixels of motion"
            scroll_pixel_sensitivity: 1.0,
        }
    }
}


