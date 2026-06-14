use {
    crate::{
        asset::{AssetManager, material::Material, mesh::Mesh},
        math::transform::Transform,
        rendering::vertex::Vertex,
        scene::{Scene, camera::Camera, entity::Entity},
    },
    glam::{Vec2, Vec3},
};

pub fn create_scene(
    device: &wgpu::Device,
    material_layout: &wgpu::BindGroupLayout,
    asset_manager: &AssetManager,
) -> Scene {
    let vertex_buffer: &[Vertex] = &[
        Vertex {
            position: Vec3::new(-1.0, 1.0, 0.0),
            uv: Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3::new(1.0, 1.0, 0.0),
            uv: Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: Vec3::new(-1.0, -1.0, 0.0),
            uv: Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3::new(1.0, -1.0, 0.0),
            uv: Vec2::new(1.0, 1.0),
        },
    ];

    let index_buffer: &[u16] = &[0, 2, 1, 1, 2, 3];

    let mut camera = Camera::new(
        Transform::default(),
        1.0,
        90.0_f32.to_radians(),
        0.1,
        100.0,
        5.0,
        180_f32.to_radians(),
    );

    camera.transform.position.z = 3.0;

    Scene {
        camera,
        entities: vec![Entity {
            mesh: Mesh::new(device, &vertex_buffer, &index_buffer),
            material: Material::new(
                device,
                material_layout,
                asset_manager.textures.get("crate.jpg").unwrap(),
            ),
        }],
    }
}
