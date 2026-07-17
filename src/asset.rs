pub mod gltf_loader;
pub mod material;
pub mod mesh;
pub mod texture;
pub mod vertex;

use {crate::asset::{material::Material, mesh::Mesh, texture::Texture}, glam::{Vec3, Vec4}};

#[derive(Clone)]
pub struct AssetManager {
    pub textures: Vec<Texture>,
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
    default_material_handle: Option<MaterialHandle>
}

#[derive(Debug, Clone, Copy)]
pub struct MeshHandle(pub u32);

#[derive(Debug, Clone, Copy)]
pub struct MaterialHandle(pub u32);

#[derive(Debug, Clone, Copy)]
pub struct TextureHandle(pub u32);

impl AssetManager {
    pub fn new() -> Self {
        Self {
            textures: vec![],
            meshes: vec![],
            materials: vec![],
            default_material_handle: None
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn default_material(&mut self) -> MaterialHandle {
        if self.default_material_handle.is_some() {
            return self.default_material_handle.unwrap();
        }
        else {
            let default_material = Material {
                name: Some("Default Material".to_string()),
                base_color_factor: Vec4::new(0.5, 0.5, 0.5, 1.0),
                roughness_factor: 1.0,
                metallic_factor: 1.0,
                metallic_roughness_texture: None,
                base_color_texture: None,
                normal_texture: None,
                occlusion_texture: None,
                emissive_factor: Vec3::new(0.0, 0.0, 0.0),
                emissive_texture: None
            };
            let handle = MaterialHandle(self.materials.len() as u32);
            self.materials.push(default_material);
            self.default_material_handle = Some(handle);
            return handle;
        }
    }

    pub fn get_texture(&self, texture_handle: TextureHandle) -> Option<&Texture> {
        self.textures.get(texture_handle.0 as usize)
    }

    pub fn get_material(&self, material_handle: MaterialHandle) -> Option<&Material> {
        self.materials.get(material_handle.0 as usize)
    }

    pub fn get_mesh(&self, mesh_handle: MeshHandle) -> Option<&Mesh> {
        self.meshes.get(mesh_handle.0 as usize)
    }
}
