mod world;
mod renderer;
mod resource_manager;
mod player;
mod state;

use std::{sync::Arc, time::Instant};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::EventLoop, window::Window};

use crate::{renderer::Renderer, resource_manager::{compress_all_regions, decompress_all_regions, load_region, save_region, ResourceManager}, state::State, world::Region};

pub struct App {
    proxy: Option<winit::event_loop::EventLoopProxy<Renderer>>,
    renderer: Option<Renderer>,
    resource_manager: Option<ResourceManager>,
    state: State,
}

impl App {
    pub fn new(event_loop: &EventLoop<Renderer>) -> Self {
        let proxy = Some(event_loop.create_proxy());
        Self {
            proxy,
            renderer: None,
            resource_manager: None,
            state: State::new()
        }
    }
}

impl ApplicationHandler<Renderer> for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = Window::default_attributes();

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        self.renderer = Some(Renderer::new(window));
        self.resource_manager = Some(ResourceManager::new());
    }

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: Renderer) {
        self.renderer = Some(event);
    }

    fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            window_id: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {
        let renderer = match &mut self.renderer {
            Some(inner) => inner,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => renderer.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                match renderer.render(&self.state, self.resource_manager.as_mut().unwrap()) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = renderer.window().inner_size();
                        renderer.resize(size.width, size.height);
                    }
                    Err(e) => {
                        log::error!("Unable to render {}", e);
                    }
                }
            }
            _ => {}
        }
    }
}

fn main() {
    env_logger::init();

    let event_loop = EventLoop::with_user_event().build().unwrap();

    let mut app = App::new(&event_loop);
    event_loop.run_app(&mut app).unwrap();
}
