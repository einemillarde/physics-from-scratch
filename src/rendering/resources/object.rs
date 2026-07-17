use {crate::scene::entity::Object, glam::Mat4, wgpu::util::DeviceExt};

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ObjectUniform {
    pub model_matrix: [[f32; 4]; 4],
    pub normal_matrix: [[f32; 4]; 4],
}

#[derive(Clone)]
pub struct ObjectGpu {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl ObjectGpu {
    pub fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> Self {
        let initial_data = ObjectUniform::default();

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Object Buffer"),
            contents: bytemuck::cast_slice(&[initial_data]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("Object Bind Group"),
        });

        Self { buffer, bind_group }
    }

    pub fn set_uniform(&self, queue: &wgpu::Queue, uniform: ObjectUniform) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[uniform]));
    }

    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(2, &self.bind_group, &[]);
    }
}

impl From<&Object> for ObjectUniform {
    fn from(object: &Object) -> Self {
        let model_matrix = Mat4::from_scale_rotation_translation(
            object.transform.scale,
            object.transform.rotation,
            object.transform.position,
        );
        let normal_matrix = model_matrix.transpose().inverse();
        Self {
            model_matrix: model_matrix.to_cols_array_2d(),
            normal_matrix: normal_matrix.to_cols_array_2d(),
        }
    }
}

impl Default for ObjectUniform {
    fn default() -> Self {
        Self {
            model_matrix: Mat4::IDENTITY.to_cols_array_2d(),
            normal_matrix: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }
}
