extern crate cgmath;
#[macro_use]
extern crate glium;
extern crate image;
extern crate rand;
extern crate rustc_serialize;
extern crate text_render_atlas;
extern crate time;

mod audio;
mod batch;
mod game_scene;
mod resources;
mod scenes;
mod state;

use glium::DisplayBuild;

use scenes::Scene;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let display = glium::glutin::WindowBuilder::new()
        .with_title("Game".to_string())
        .with_depth_buffer(24)
        //.with_vsync()
        .build_glium()
        .unwrap();

    let batch_system = batch::BatchSystem::new(&display);

    let mut audio_engine = audio::AudioEngine::new();
    let mut scene: Box<Scene> = Box::new(game_scene::GameScene::new(&display));

    loop {
        let mut events = Vec::new();
        for event in display.poll_events() {
            match event {
                glium::glutin::Event::Closed => return,
                e => events.push(e),
            }
        }

        let mut target = display.draw();
        let mut audio_state = audio::AudioState::default();
        let new_scene = scene.next_frame(&mut target, &batch_system, &events, &mut audio_state);
        audio_engine.synchronize(&audio_state);
        target.finish().unwrap();

        if let Some(new_scene) = new_scene {
            scene = new_scene;
        }
    }
}
