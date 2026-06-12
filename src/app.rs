use {crate::{rendering::renderer::Renderer, scene::Scene}, std::sync::Arc, winit::window::Window};

pub mod application_handler;
pub mod input;

#[cfg(target_arch = "wasm32")]
use {wasm::bindgen::prelude::*, winit::platform::web::EventLoopExtWebSys};

pub struct App {
    #[cfg(target_arch = "wasm32")]
    proxy: Option<winit::event_loop::EventLoopProxy<State>>,
    renderer: Option<Renderer>,
    window: Option<Arc<Window>>,
    scene: Option<Scene>
}

impl App {
    pub fn new(#[cfg(target_arch = "wasm32")] event_loop: &EventLoop<State>) -> Self {
        #[cfg(target_arch = "wasm32")]
        let proxy = Some(event_loop.create_proxy());

        Self {
            #[cfg(target_arch = "wasm32")]
            proxy,
            renderer: None,
            window: None,
            scene: None
        }
    }
}
