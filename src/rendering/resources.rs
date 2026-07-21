pub mod material;
pub mod mesh;
pub mod object;
pub mod texture;

use {
    crate::{
        asset::{AssetManager, MaterialHandle, MeshHandle, TextureHandle, texture::Texture},
        scene::{ObjectHandle, Scene},
    },
    material::MaterialGpu,
    mesh::MeshGpu,
    object::{ObjectGpu, ObjectUniform},
    texture::TextureGpu,
};

#[derive(Clone)]
pub struct GpuResources {
    pub meshes: Vec<MeshGpu>,
    pub textures: Vec<TextureGpu>,
    pub materials: Vec<MaterialGpu>,
    pub objects: Vec<ObjectGpu>,
}

impl GpuResources {
    pub fn new() -> Self {
        Self {
            meshes: vec![],
            textures: vec![],
            materials: vec![],
            objects: vec![],
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn load_assets(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        asset_manager: &AssetManager,
        scene: &Scene,
        material_layout: &wgpu::BindGroupLayout,
        object_layout: &wgpu::BindGroupLayout,
    ) -> anyhow::Result<()> {
        self.reset();

        let default_base_color_texture =
            TextureGpu::new(device, queue, &Texture::new(1, 1, vec![231, 231, 231, 255]))?;

        let default_metallic_roughness_texture =
            TextureGpu::new(device, queue, &Texture::new(1, 1, vec![0, 0, 128, 255]))?;

        let default_normal_texture =
            TextureGpu::new(device, queue, &Texture::new(1, 1, vec![128, 128, 255, 255]))?;

        let default_occlusion_texture =
            TextureGpu::new(device, queue, &Texture::new(1, 1, vec![0, 0, 0, 255]))?;

        let default_emissive_texture =
            TextureGpu::new(device, queue, &Texture::new(1, 1, vec![0, 0, 0, 255]))?;

        for mesh in asset_manager.meshes.iter() {
            self.meshes.push(MeshGpu::new(device, &mesh));
        }

        for texture in asset_manager.textures.iter() {
            self.textures
                .push(TextureGpu::new(device, queue, &texture)?);
        }

        for material in asset_manager.materials.iter() {
            self.materials.push(MaterialGpu::new(
                device,
                material_layout,
                &material,
                &self,
                &default_base_color_texture,
                &default_metallic_roughness_texture,
                &default_normal_texture,
                &default_occlusion_texture,
                &default_emissive_texture,
            )?);
        }

        for object in &scene.objects {
            let uniform = ObjectUniform::from(object);
            let gpu_resource = ObjectGpu::new(device, object_layout);
            gpu_resource.set_uniform(queue, uniform);
            self.objects.push(gpu_resource);
        }

        Ok(())
    }

    pub fn get_mesh(&self, handle: MeshHandle) -> Option<&MeshGpu> {
        self.meshes.get(handle.0 as usize)
    }

    pub fn get_material(&self, handle: MaterialHandle) -> Option<&MaterialGpu> {
        self.materials.get(handle.0 as usize)
    }

    pub fn get_texture(&self, handle: TextureHandle) -> Option<&TextureGpu> {
        self.textures.get(handle.0 as usize)
    }

    pub fn get_object(&self, handle: ObjectHandle) -> Option<&ObjectGpu> {
        self.objects.get(handle.0 as usize)
    }
}
