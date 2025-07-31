mod world;
mod renderer;

use std::sync::Arc;

use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::EventLoop, window::Window};

use crate::{renderer::Renderer, world::Chunk};

pub struct App {
    proxy: Option<winit::event_loop::EventLoopProxy<Renderer>>,
    renderer: Option<Renderer>,
}

impl App {
    pub fn new(event_loop: &EventLoop<Renderer>) -> Self {
        let proxy = Some(event_loop.create_proxy());
        Self {
            renderer: None,
            proxy
        }
    }
}

impl ApplicationHandler<Renderer> for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = Window::default_attributes();

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        self.renderer = Some(Renderer::new(window));
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
                renderer.render();
            }
            _ => {}
        }
    }
}

fn main() {
    env_logger::init();
    
    let world = vec![vec![vec![Chunk::new()]]];

    let event_loop = EventLoop::with_user_event().build().unwrap();

    let mut app = App::new(&event_loop);
    event_loop.run_app(&mut app).unwrap();
}
