pub mod camera;
pub mod entity;
pub mod scene_1;

use {camera::Camera, entity::Entity};

pub struct Scene {
    pub entities: Vec<Entity>,
    pub camera: Camera,
}

impl Scene {
    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        for entity in self.entities.iter() {
            entity.render(render_pass);
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.camera.update(dt)
    }
}
