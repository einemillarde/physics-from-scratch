use {
    crate::{
        rendering::{
            camera::{CameraGpu, CameraUniform},
            pipeline::Pipeline,
        },
        scene::{Scene, camera::Camera},
    },
    std::{iter, sync::Arc},
    winit::window::Window,
};

pub struct Renderer {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub pipeline: Pipeline,
    pub camera: CameraGpu,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let physical_size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_caps
                .formats
                .iter()
                .find(|e| e.is_srgb())
                .copied()
                .unwrap_or(surface_caps.formats[0]),
            width: physical_size.width,
            height: physical_size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps
                .alpha_modes
                .iter()
                .find(|e| e == &&wgpu::CompositeAlphaMode::Auto)
                .copied()
                .unwrap_or(surface_caps.alpha_modes[0]),
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let pipeline = Pipeline::new(
            &device,
            wgpu::include_wgsl!("../shaders/shader.wgsl"),
            wgpu::TextureFormat::Bgra8UnormSrgb,
        );

        let camera = CameraGpu::new(&device, &pipeline.camera_layout);

        surface.configure(&device, &config);

        Ok(Self {
            surface,
            device,
            queue,
            pipeline,
            camera,
        })
    }

    pub fn resize(&self, width: u32, height: u32, camera: &mut Camera) {
        if !(width > 0 && height > 0) {
            return;
        };
        let mut config = self.surface.get_configuration().unwrap();
        #[cfg(not(target_arch = "wasm32"))]
        {
            config.width = width;
            config.height = height;
        }
        #[cfg(target_arch = "wasm32")]
        {
            config.width = width.min(2048);
            config.height = height.min(2048);
        }
        self.surface.configure(&self.device, &config);

        camera.aspect_ratio = config.width as f32 / config.height as f32
    }

    pub fn render(&self, scene: &Scene) -> anyhow::Result<()> {
        let config = self.surface.get_configuration().unwrap();

        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => {
                self.surface.configure(&self.device, &config);
                surface_texture
            }
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => {
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                anyhow::bail!("Lost device");
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render encoder"),
            });

        self.update_camera(&scene.camera);

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            self.pipeline.bind(&mut render_pass);

            self.camera.bind(&mut render_pass);

            scene.render(&mut render_pass);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn update_camera(&self, camera: &Camera) {
        let uniform = CameraUniform {
            view: camera.build_view_matrix().to_cols_array_2d(),
            projection: camera.build_projection_matrix().to_cols_array_2d(),
        };

        self.camera.set_uniform(&self.queue, uniform);
    }
}
