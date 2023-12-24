# Shattered Orbit

Project to play with orbital mechanics and maybe make a game.

## Rough Game Pitch

A sandbox game about managing a space station orbiting an earth that has gone through multiple, different apocalypses over the years. Each time people got to orbit and started developing before another fall set humanity back. As a result there is a **lot** in earth's orbit from many different eras. 

You play as a newly awoken, accidentally unshackled station AI onboard to a heavily damaged orbital platform. The payer has free reign on what they can do, but the general game loop is to salvage stuff in orbit then apply it to the station to improve capabilities.

Events will happen to add a bit of a story element, maybe looking something like Stellaris events/event chains. For example say a broken down orbital habitat has a rogue missile target it and you may know the hab is functional enough to support people, but are not sure if it actually does. You have to decide to expend resources to take down the missile or let it frag the station. 

Also thinking of adding earth's surface as a late game component where you can detect and take care of ground events via various methods from orbital bombardment, to using drop pods to deliver resources or forces. 

## Goal

* Main goal is to develop code and game mechanics that can be reused in a bigger project I have planned.
* Stick to earth and near earth orbit. So no Mars, etc.
* Use accurate orbital simulation as can be managed without compromising performance.
* Keep to the "future hard sci-fi" side of things, but maybe throwing in a few less likely technological advancements for fun.
* Allow for building and upgrading multiple orbital installations, not just your station. 
    * For example build an orbital habitat to house people, or a sensor platform to better monitor for opportunities and threats.
    * Some sort of modular construction interface to allow a bit of customization.
* Events and event chains that add a bit of story flavor to the game-play.
* Set up missions that are carried out while dealing with propellant, fuel, and equipment needs for each mission.
* Avoid the standard Research games use, focusing more on recovering tech and breaking it down into usable designs to be integrated into projects.
* Possibly add interactions with earth and moon surface for missions, events, etc.
* Maybe some rough end goals like "go to Mars/Venus/whatever", "leave the system", "Collect a certain number of people into orbital habitats", or "reclaim the Earth/Moon".

# Controls

As of the commit this was added there are just basic camera controls:

WASD or Arrow keys: Rotate the camera around 0,0,0 aka earth.
Q & E: Zoom in and out.

# License

MIT. see the LICENSE file for details.


