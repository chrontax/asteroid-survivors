mod input;
mod render;

pub mod physics;
pub mod text;

use std::time::Instant;

use anyhow::Result;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Fullscreen, Window, WindowId},
};

use self::render::Renderer;

pub use input::Input;
pub use render::{EverythingToDraw, RenderLiteral, ShapeLiteral};

pub trait Game {
    fn init() -> (EngineInitInfo, Self);
    fn update(&mut self, dt: f32);
    fn input(&mut self, input: Input);
    fn draw(&self) -> EverythingToDraw;
}

pub struct EngineInitInfo {
    pub windowed: bool,
    pub resizeable: bool,
    pub resolution: PhysicalSize<u32>,
}

// TODO: better name
pub struct MainEngineThing<G: Game> {
    last_wait: Option<Instant>,
    window: Option<Window>,
    renderer: Renderer,
    game: Option<G>,
}

impl<G: Game> Default for MainEngineThing<G> {
    fn default() -> Self {
        Self {
            last_wait: None,
            window: None,
            renderer: Renderer::default(),
            game: None,
        }
    }
}

impl<G: Game> ApplicationHandler for MainEngineThing<G> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.last_wait = Some(Instant::now());
        let (init_info, game) = G::init();
        self.game = Some(game);
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_resizable(init_info.resizeable)
                    .with_inner_size(init_info.resolution)
                    .with_fullscreen(if init_info.windowed {
                        None
                    } else {
                        Some(Fullscreen::Borderless(None))
                    }),
            )
            .unwrap();
        self.renderer.init(&window).unwrap();
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                self.renderer
                    .render(
                        &self.game.as_ref().unwrap().draw(),
                        self.window.as_ref().unwrap(),
                    )
                    .unwrap();
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::Resized(_) => self.renderer.resized = true,
            other => {
                if let Ok(input) = other.try_into() {
                    self.game.as_mut().unwrap().input(input);
                }
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.game.as_mut().unwrap().update(
            self.last_wait
                .replace(Instant::now())
                .unwrap()
                .elapsed()
                .as_secs_f32(),
        );
    }
}

pub fn run_game<G: Game>() -> Result<()> {
    EventLoop::new()?.run_app(&mut MainEngineThing::<G>::default())?;
    Ok(())
}
