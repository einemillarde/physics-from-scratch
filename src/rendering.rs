pub mod app;
pub mod math;
pub mod state;

use {app::App, glam::Vec3, math::vertex::Vertex, winit::event_loop::EventLoop};

const VERTICES: &[Vertex] = &[
    Vertex {
        position: Vec3::new(-0.0868241, 0.49240386, 0.0),
        color: Vec3::new(0.5, 0.0, 0.5),
    },
    Vertex {
        position: Vec3::new(-0.49513406, 0.06958647, 0.0),
        color: Vec3::new(0.5, 0.0, 0.5),
    },
    Vertex {
        position: Vec3::new(-0.21918549, -0.44939706, 0.0),
        color: Vec3::new(0.5, 0.0, 0.5),
    },
    Vertex {
        position: Vec3::new(0.35966998, -0.3473291, 0.0),
        color: Vec3::new(0.5, 0.0, 0.5),
    },
    Vertex {
        position: Vec3::new(0.44147372, 0.2347359, 0.0),
        color: Vec3::new(0.5, 0.0, 0.5),
    },
];

const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

pub fn run() -> anyhow::Result<()> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
    }
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init_with_level(log::Level::Info).unwrap_throw();
    }

    let event_loop = EventLoop::with_user_event().build()?;
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut app = App::new();
        event_loop.run_app(&mut app)?;
    }
    #[cfg(target_arch = "wasm32")]
    {
        let app = App::new(&event_loop);
        event_loop.spawn_app(app);
    }

    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    run().unwrap_throw();

    Ok(())
}
