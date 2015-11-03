use std::sync::mpsc::Receiver;

use cgmath;
use glium;
use glium::glutin::Event;

use audio::AudioState;
use batch::BatchSystem;
use game_scene::view::GameView;
use scenes::Scene;
use state;
use state::GameState;

pub struct GameScene {
    inner: GameSceneImpl
}

enum GameSceneImpl {
    Loading {
        view: Receiver<GameView>,
    },
    Game {
        view: GameView,
        state: GameState,
        render_ticks: u64,
    },
}

impl GameScene {
    pub fn new(display: &glium::Display) -> GameScene {
        let rules = state::rules::load(None);

        GameScene {
            inner: GameSceneImpl::Game {
                view: GameView::new(display, 8),
                state: GameState::new(rules),
                render_ticks: 0,
            },
        }
    }
}

impl Scene for GameScene {
    fn next_frame(&mut self, target: &mut glium::Frame, batch: &BatchSystem, events: &[Event],
                  audio: &mut AudioState) -> Option<Box<Scene>>
    {
        let new_view = match self.inner {
            GameSceneImpl::Loading { ref view } => {
                match view.try_recv() {
                    Ok(v) => v,
                    Err(_) => { return None; }
                }
            },
            GameSceneImpl::Game { ref mut view, ref mut state, ref mut render_ticks } => {
                view.process(target, batch, events, audio, state);
                *render_ticks += 1;
                return None;
            },
        };

        // if we reach here, we switch from loading screen to game screen

        panic!();

        /*self.inner = GameSceneImpl::Game {
            view: new_view,
            state: GameState,
        };

        None*/
    }
}
