use bevy::{
    prelude::{ResMut, Time},
    utils::Instant,
};


//Simulation(Sim) time affects calculations for all simulation systems like orbital movments.
//  It's seprate from the normal Time<Virtual> so it does not affect things like animations or UI
//  Sim time works with very large numbers, representing the passage of days to months per second.


//Simple system that just makes sure the sim time gets advanced each update
pub fn advance_sim_time(mut time: ResMut<Time<SimTime>>) {
    time.update_from_external(Instant::now());
}



//How fast we run per second
#[derive(Debug, Clone, Copy)]
pub enum SimSpeed {
    Paused = 0,
    Hour = 3600,
    Day = 86400,
    Week = 604800,
    Month = 18748800,
}

//Context for the clock we use to control sim time passage
#[derive(Debug)]
pub struct SimTime {
    last_external_time: Instant,
    sim_speed: SimSpeed,
}

impl Default for SimTime {
    fn default() -> Self {
        Self {
            last_external_time: Instant::now(),
            sim_speed: SimSpeed::Week,
        }
    }
}

//Logic on how the sim time is advanced. must be a seprate trait so we can attach it to Time<T>
trait SimTimeLogic {
    fn update_from_external(&mut self, instant: Instant);
}

impl SimTimeLogic for Time<SimTime> {
    fn update_from_external(&mut self, instant: Instant) {
         let delta = (instant - self.context().last_external_time) * self.context().sim_speed as u32;
         self.advance_by(delta);
         self.context_mut().last_external_time = instant;
    }
}
