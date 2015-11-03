/// A scene is an object that is at the foreground of the window. It is this object that chooses
/// what is drawn and what is played in the speakers. It receives user input.
///
/// STATUS: needs better input handling
use glium::Surface;
use glium::Frame;
use glium::glutin::Event;

use audio::AudioState;

use batch::BatchSystem;

/// Trait for objects that can fulfill the role of being at the foreground of the game.
pub trait Scene {
    /// Draws a frame on the surface.
    fn next_frame(&mut self, &mut Frame, &BatchSystem, &[Event], &mut AudioState)
                  -> Option<Box<Scene>>;
}
