pub mod camera;
pub mod entity;

use {
    crate::{asset::AssetManager, rendering::resources::GpuResources},
    camera::Camera,
    entity::Object,
};

#[derive(Clone)]
pub struct Scene {
    pub objects: Vec<Object>,
    pub camera: Camera,
}

#[derive(Debug, Clone, Copy)]
pub struct ObjectHandle(pub u32);

impl Scene {
    pub fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        gpu_resources: &'a GpuResources,
        asset_manager: &'a AssetManager,
    ) {
        for object in self.objects.iter() {
            object.render(render_pass, gpu_resources, asset_manager);
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.camera.update(dt)
    }
}
