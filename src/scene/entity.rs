use crate::{
    asset::{AssetManager, MeshHandle},
    math::transform::Transform,
    rendering::resources::GpuResources,
    scene::ObjectHandle,
};

#[derive(Clone, Debug)]
pub struct Object {
    pub mesh: Option<MeshHandle>,
    pub transform: Transform,
    pub children: Vec<ObjectHandle>,
    pub resource_handle: ObjectHandle
}

impl Object {
    pub fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        gpu_resources: &'a GpuResources,
        asset_manager: &'a AssetManager,
    ) {
        let mesh = asset_manager.get_mesh(self.mesh.unwrap()).unwrap();
        let material_handle = mesh.material;
        let mesh_resource = gpu_resources.get_mesh(self.mesh.unwrap()).unwrap();
        let material_resource = gpu_resources.get_material(material_handle).unwrap();
        let object_resource = gpu_resources.get_object(self.resource_handle).unwrap();

        material_resource.bind(render_pass);
        object_resource.bind(render_pass);
        mesh_resource.draw(render_pass);
    }
}
