use winit::{dpi::PhysicalPosition, event_loop::ActiveEventLoop, keyboard::KeyCode};

pub struct Input;

impl Input {
    pub fn handle_keyboard_input(event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            _ => {}
        }
    }

    pub fn handle_cursor_moved(
        _event_loop: &ActiveEventLoop,
        _physical_position: PhysicalPosition<f64>,
    ) {
    }
}
