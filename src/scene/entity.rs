use crate::asset::{material::Material, mesh::Mesh};

pub struct Entity {
    pub mesh: Mesh,
    pub material: Material,
}

impl Entity {
    pub fn render<'a>(&'a self, rendering_pass: &mut wgpu::RenderPass<'a>) {
        self.material.bind(rendering_pass);
        self.mesh.draw(rendering_pass);
    }
}
