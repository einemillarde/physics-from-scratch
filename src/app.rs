use {
    crate::{asset::AssetManager, rendering::renderer::Renderer, scene::Scene},
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

pub struct App {
    renderer: Option<Renderer>,
    window: Option<Arc<Window>>,
    scene: Option<Scene>,
    asset_manager: Option<AssetManager>,
    last_update: Instant,
}

impl App {
    pub fn new() -> Self {
        Self {
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

        self.window = Some(Arc::new(
            event_loop.create_window(window_attributes).unwrap(),
        ));

        self.renderer =
            Some(pollster::block_on(Renderer::new(self.window.as_ref().unwrap().clone())).unwrap());

        let renderer = self.renderer.as_mut().unwrap();

        self.asset_manager = Some(AssetManager::new());

        let asset_manager = self.asset_manager.as_mut().unwrap();

        self.scene = Some(asset_manager
            .load_gltf("assets/test_scene_1.gltf").unwrap());

        renderer
            .resources
            .load_assets(
                &renderer.device,
                &renderer.queue,
                asset_manager,
                self.scene.as_ref().unwrap(),
                &renderer.pipeline.material_layout,
                &renderer.pipeline.object_layout,
            )
            .unwrap();
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: Renderer) {
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
            WindowEvent::Resized(size) => self.renderer.as_mut().unwrap().resize(
                size.width,
                size.height,
                &mut self.scene.as_mut().unwrap().camera,
            ),
            WindowEvent::RedrawRequested => {
                self.update();
                self.window.as_ref().unwrap().request_redraw();
                self.renderer
                    .as_ref()
                    .unwrap()
                    .render(
                        self.scene.as_ref().unwrap(),
                        self.asset_manager.as_ref().unwrap(),
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
            } => self.handle_keyboard_input(event_loop, code, key_state.is_pressed()),
            WindowEvent::CursorMoved {
                position: physical_position,
                ..
            } => self.handle_cursor_moved(event_loop, physical_position),
            _ => {}
        }
    }
}
