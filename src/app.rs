use {
    crate::{asset::AssetManager, rendering::renderer::Renderer, scene, scene::Scene},
    std::sync::Arc,
    std::time::Instant,
    winit::{
        application::ApplicationHandler,
        dpi::PhysicalPosition,
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
    last_update: Instant,
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
            last_update: Instant::now(),
        }
    }

    fn update(&mut self) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        let scene = self.scene.as_mut().unwrap();

        scene.update(dt);
    }

    fn handle_keyboard_input(
        &mut self,
        _event_loop: &ActiveEventLoop,
        code: winit::keyboard::KeyCode,
        is_pressed: bool,
    ) {
        let scene = self.scene.as_mut().unwrap();
        scene.camera.handle_keyboard_input(code, is_pressed);
    }

    fn handle_cursor_moved(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _physical_position: PhysicalPosition<f64>,
    ) {
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

        self.scene = Some(scene::scene_1::create_scene(
            &renderer.device,
            &renderer.pipeline.material_layout,
            asset_manager,
        ));
        self.last_update = Instant::now();

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
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => self.renderer.as_ref().unwrap().resize(
                size.width,
                size.height,
                &mut self.scene.as_mut().unwrap().camera,
            ),
            WindowEvent::RedrawRequested => {
                self.update();
                self.window.as_ref().unwrap().request_redraw();
                let _ = self
                    .renderer
                    .as_ref()
                    .unwrap()
                    .render(self.scene.as_ref().unwrap())
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
            } => self.handle_keyboard_input(event_loop, code, key_state.is_pressed()),
            WindowEvent::CursorMoved {
                position: physical_position,
                ..
            } => self.handle_cursor_moved(event_loop, physical_position),
            _ => {}
        }
    }
}
