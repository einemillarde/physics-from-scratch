use {app::App, winit::event_loop::EventLoop};

pub mod app;
pub mod asset;
pub mod math;
pub mod rendering;
pub mod scene;

pub fn run() -> anyhow::Result<()> {
    env_logger::init();
    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;
    Ok(())
}
