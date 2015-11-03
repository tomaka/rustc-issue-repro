use resources;
use resources::ResourceId;

#[derive(Debug, Default)]
pub struct AudioState {
    pub sounds: Vec<ResourceId>,
    pub one_shot_sounds: Vec<ResourceId>,
}

pub struct AudioEngine(());

impl AudioEngine {
    pub fn new() -> AudioEngine {
        AudioEngine(())
    }

    pub fn synchronize(&mut self, state: &AudioState) {
    }
}
