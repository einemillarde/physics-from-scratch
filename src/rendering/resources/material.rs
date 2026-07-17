use {
    crate::{asset::material::Material, rendering::resources::{GpuResources, texture::TextureGpu}},
    wgpu::util::DeviceExt,
};

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MaterialUniform {
    pub base_color_factor: [f32; 4],

    pub emissive_factor: [f32; 3],
    pub _pad0: f32,

    pub metallic_factor: f32,
    pub roughness_factor: f32,

    pub _pad1: [f32; 2],
}

#[derive(Clone)]
pub struct MaterialGpu {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl MaterialGpu {
    pub fn new(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        material: &Material,
        gpu_resources: &GpuResources,
        default_white: &TextureGpu,
        default_normal: &TextureGpu,
        default_black: &TextureGpu,
    ) -> anyhow::Result<Self> {
        let uniform = MaterialUniform::from(material);

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Material Buffer"),
            contents: bytemuck::bytes_of(&uniform),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let base_color_texture = material
            .base_color_texture
            .and_then(|h| gpu_resources.get_texture(h))
            .unwrap_or(default_white);

        let normal_texture = material
            .normal_texture
            .and_then(|h| gpu_resources.get_texture(h))
            .unwrap_or(default_normal);

        let emissive_texture = material
            .emissive_texture
            .and_then(|h| gpu_resources.get_texture(h))
            .unwrap_or(default_white);

        let occlusion_texture = material
            .occlusion_texture
            .and_then(|h| gpu_resources.get_texture(h))
            .unwrap_or(default_black);

        let metallic_roughness_texture = material
            .metallic_roughness_texture
            .and_then(|h| gpu_resources.get_texture(h))
            .unwrap_or(default_white);

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Material Bind Group"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&base_color_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&metallic_roughness_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&normal_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::TextureView(&occlusion_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::TextureView(&emissive_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 6,
                    resource: wgpu::BindingResource::Sampler(&base_color_texture.sampler),
                },
            ],
        });

        Ok(Self { buffer, bind_group })
    }

    pub fn set_uniform(&self, queue: &wgpu::Queue, uniform: MaterialUniform) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[uniform]));
    }

    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(0, &self.bind_group, &[]);
    }
}

impl From<&Material> for MaterialUniform {
    fn from(material: &Material) -> Self {
        Self {
            base_color_factor: material.base_color_factor.to_array(),

            emissive_factor: material.emissive_factor.to_array(),
            _pad0: 0.0,

            metallic_factor: material.metallic_factor,
            roughness_factor: material.roughness_factor,

            _pad1: [0.0; 2],
        }
    }
}
