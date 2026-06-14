use {
    crate::{
        asset::{AssetManager, material::Material, mesh::Mesh},
        rendering::{renderer::Renderer, vertex::Vertex},
        scene::{Scene, entity::Entity},
    },
    glam::{Vec2, Vec3},
};

pub fn create_scene(renderer: &Renderer, asset_manager: &AssetManager) -> Scene {
    let vertex_buffer: &[Vertex] = &[
        Vertex {
            position: Vec3::new(-0.5, 0.5, 0.0),
            uv: Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3::new(0.5, 0.5, 0.0),
            uv: Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: Vec3::new(-0.5, -0.5, 0.0),
            uv: Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3::new(0.5, -0.5, 0.0),
            uv: Vec2::new(1.0, 1.0),
        },
    ];

    let index_buffer: &[u16] = &[0, 2, 1, 1, 2, 3];

    Scene {
        entities: vec![Entity {
            mesh: Mesh::new(&renderer.device, &vertex_buffer, &index_buffer),
            material: Material::new(
                &renderer.device,
                &renderer.pipeline.material_layout,
                asset_manager.textures.get("crate.jpg").unwrap(),
            ),
        }],
    }
}
