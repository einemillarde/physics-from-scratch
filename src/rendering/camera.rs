use {crate::scene::camera::Camera, glam::Mat4, wgpu::util::DeviceExt};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub position: [f32; 3],
    pub _pad0: u32,
    pub view: [[f32; 4]; 4],
    pub projection: [[f32; 4]; 4],
}

pub struct CameraGpu {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl CameraGpu {
    pub fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> Self {
        let initial_data = CameraUniform::default();

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[initial_data]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("Camera Bind Group"),
        });

        Self { buffer, bind_group }
    }

    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(1, &self.bind_group, &[]);
    }

    pub fn update(&self, queue: &wgpu::Queue, camera: &Camera) {
        let uniform = CameraUniform {
            position: camera.transform.position.to_array(),
            _pad0: 0,
            view: camera.build_view_matrix().to_cols_array_2d(),
            projection: camera.build_projection_matrix().to_cols_array_2d(),
        };

        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[uniform]));
    }
}

impl Default for CameraUniform {
    fn default() -> Self {
        Self {
            position: [0.0; 3],
            _pad0: 0,
            view: Mat4::IDENTITY.to_cols_array_2d(),
            projection: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }
}
