use wgpu::util::DeviceExt;

use crate::scene::light::Light;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct LightData {
    pub position_or_direction: [f32; 3],
    pub light_type: u32,

    pub color: [f32; 3],
    pub intensity: f32,

    pub _pad0: [u32; 3],
    pub range: f32,
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct LightCount {
    pub _pad0: [u32; 3],
    pub count: u32,
}

pub struct LightGpu {
    pub buffer: wgpu::Buffer,
    pub count_buffer: wgpu::Buffer,

    pub bind_group: wgpu::BindGroup,

    pub count: u32,
}

impl LightGpu {
    pub fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> Self {
        let light_data = vec![LightData::default()];

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Light Buffer"),
            contents: bytemuck::cast_slice(&light_data),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let count_data = LightCount {
            count: 1,
            _pad0: [0; 3],
        };

        let count_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Light Count Buffer"),
            contents: bytemuck::bytes_of(&count_data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Light Bind Group"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: count_buffer.as_entire_binding(),
                },
            ],
        });

        Self {
            buffer,
            count_buffer,
            bind_group,
            count: 1,
        }
    }

    pub fn update(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        lights: &[Light],
    ) {
        if lights.len() as u32 != self.count {
            let light_data: Vec<LightData> = lights.iter().map(LightData::from).collect();

            self.buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Light Storage Buffer"),
                contents: bytemuck::cast_slice(&light_data),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            });

            self.count = lights.len() as u32;

            let count = LightCount {
                count: self.count,
                _pad0: [0; 3],
            };

            queue.write_buffer(&self.count_buffer, 0, bytemuck::bytes_of(&count));

            self.bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Light Bind Group"),
                layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: self.count_buffer.as_entire_binding(),
                    },
                ],
            });
        } else {
            let light_data: Vec<LightData> = lights.iter().map(LightData::from).collect();

            queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&light_data));
        }
    }

    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(3, &self.bind_group, &[]);
    }
}

impl From<&Light> for LightData {
    fn from(light: &Light) -> Self {
        match light {
            Light::Point(point) => LightData {
                position_or_direction: point.position.to_array(),
                color: point.color.to_array(),
                intensity: point.intensity,
                range: point.range,
                light_type: 0,
                _pad0: [0; 3],
            },
            Light::Directional(directional) => LightData {
                position_or_direction: directional.direction.to_array(),
                color: directional.color.to_array(),
                intensity: directional.intensity,
                range: 0.0,
                light_type: 1,
                _pad0: [0; 3],
            },
        }
    }
}
