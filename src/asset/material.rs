use {
    crate::asset::TextureHandle,
    glam::{Vec3, Vec4},
};

#[derive(Clone, Debug)]
pub struct Material {
    pub name: Option<String>,

    pub base_color_factor: Vec4,
    pub base_color_texture: Option<TextureHandle>,

    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub metallic_roughness_texture: Option<TextureHandle>,

    pub normal_texture: Option<TextureHandle>,

    pub occlusion_texture: Option<TextureHandle>,

    pub emissive_factor: Vec3,
    pub emissive_texture: Option<TextureHandle>,
}
