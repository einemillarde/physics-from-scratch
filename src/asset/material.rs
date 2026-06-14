use crate::asset::texture::Texture;

pub struct Material {
    pub bind_group: wgpu::BindGroup,
}

impl Material {
    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(0, &self.bind_group, &[]);
    }

    pub fn new(
        device: &wgpu::Device,
        material_layout: &wgpu::BindGroupLayout,
        texture: &Texture,
    ) -> Self {
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Material Bind Group"),
            layout: material_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
        });

        Self { bind_group }
    }
}
