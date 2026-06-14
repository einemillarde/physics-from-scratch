use {
    crate::{
        input::Input,
        asset::AssetManager,
        rendering::renderer::Renderer,
        scene::Scene,
        scene
    },
    std::sync::Arc,
    winit::{
        application::ApplicationHandler,
        event::{KeyEvent, WindowEvent},
        event_loop::ActiveEventLoop,
        keyboard::PhysicalKey,
        window::Window,
    },
};

#[cfg(target_arch = "wasm32")]
use {wasm::bindgen::prelude::*, winit::platform::web::EventLoopExtWebSys};

pub struct App {
    #[cfg(target_arch = "wasm32")]
    proxy: Option<winit::event_loop::EventLoopProxy<State>>,
    renderer: Option<Renderer>,
    window: Option<Arc<Window>>,
    scene: Option<Scene>,
    asset_manager: Option<AssetManager>,
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
            scene: None,
            asset_manager: None,
        }
    }
}

impl ApplicationHandler<Renderer> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes();

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowAttributesExtWebSys;

            const CANVAS_ID: &str = "canvas";

            let window = wgpu::web_sys::window().unwrap_throw();
            let document = window.document().unwrap_throw();
            let canvas = document.get_element_by_id(CANVAS_ID).unwrap_throw();
            let html_canvas_element = canvas.unchecked_into();
            window_attributes = window_attributes.with_canvas(Some(html_canvas_element));
        }

        self.window = Some(Arc::new(
            event_loop.create_window(window_attributes).unwrap(),
        ));

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.renderer = Some(
                pollster::block_on(Renderer::new(self.window.as_ref().unwrap().clone())).unwrap(),
            );
        }

        let renderer = self.renderer.as_ref().unwrap();

        self.asset_manager =
            Some(AssetManager::new().load_default_assets(&renderer.device, &renderer.queue));

        let asset_manager = self.asset_manager.as_ref().unwrap();

        self.scene = Some(scene::scene_1::create_scene(&renderer, asset_manager));

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(proxy) = self.proxy.take() {
                wasm_bindgen_futures::spawn_local(async move {
                    assert!(
                        proxy
                            .send_event(
                                State::new(self.window)
                                    .await
                                    .expect("Unable to create canvas!!!")
                            )
                            .is_ok()
                    )
                });
            }
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: Renderer) {
        #[cfg(target_arch = "wasm32")]
        {
            event.window.request_redraw();
            event.resize(
                event.window.inner_size().width,
                event.window.inner_size().height,
            );
        }
        self.renderer = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let renderer = match &mut self.renderer {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => renderer.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
                let _ = renderer
                    .render(
                        self.scene.as_ref().unwrap(),
                    )
                    .unwrap();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => Input::handle_keyboard_input(event_loop, code, key_state.is_pressed()),
            WindowEvent::CursorMoved {
                position: physical_position,
                ..
            } => Input::handle_cursor_moved(event_loop, physical_position),
            _ => {}
        }
    }
}
