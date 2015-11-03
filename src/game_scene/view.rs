use glium;
use glium::glutin::Event;
use cgmath::Vector2;

use audio::AudioState;
use batch::BatchSystem;

use game_scene::camera::CameraState;
use game_scene::map::MapDrawSystem;
use game_scene::models::ModelsSystem;
use game_scene::ui;
use state::GameState;

pub struct GameView {
    camera: CameraState,
    map_draw: MapDrawSystem,
    climate_map: glium::Texture2d,
    colors_pixel_buffer: glium::texture::pixel_buffer::PixelBuffer<(u8, u8, u8)>,
    colors_map: glium::Texture2d,
    models_system: ModelsSystem,
    ui_state: ui::UiState,
    ui_cursor_position: Option<[f32; 2]>,
}

impl GameView {
    pub fn new(display: &glium::Display, max_simulation_speed_level: u8) -> GameView {
        GameView {
            camera: CameraState::new(display.get_framebuffer_dimensions()),
            map_draw: MapDrawSystem::new(display),
            climate_map: glium::Texture2d::empty(display, 1603, 509).unwrap(),
            colors_pixel_buffer: glium::texture::pixel_buffer::PixelBuffer::new_empty(display, 1603 * 509),
            colors_map: glium::Texture2d::empty_with_format(display, glium::texture::UncompressedFloatFormat::U8U8U8, glium::texture::MipmapsOption::NoMipmap, 1603, 509).unwrap(),
            models_system: ModelsSystem::new(display),
            ui_state: ui::UiState {
                nation_name: "no name".to_owned(),
                simulation_speed_level: 0,
                max_simulation_speed_level: max_simulation_speed_level,
            },
            ui_cursor_position: None,
        }
    }

    /// Returns the level of the simulation speed, between 0 and <undefined value>.
    #[inline]
    pub fn get_simulation_speed_level(&self) -> u8 {
        self.ui_state.simulation_speed_level
    }

    pub fn process<T>(&mut self, target: &mut T, batch_system: &BatchSystem, events: &[Event],
                      audio: &mut AudioState, state: &mut GameState)
        where T: glium::Surface
    {
    }
}
