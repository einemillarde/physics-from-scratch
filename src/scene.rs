pub mod entity;
pub mod scene_1;

use entity::Entity;

pub struct Scene {
    pub entities: Vec<Entity>,
}

impl Scene {
    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        for entity in self.entities.iter() {
            entity.render(render_pass);
        }
    }
}
