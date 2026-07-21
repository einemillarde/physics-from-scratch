use {
    crate::{
        asset::{
            AssetManager, MaterialHandle, MeshHandle, TextureHandle, material::Material, mesh::Mesh, texture::{ColorSpace, Texture}, vertex::Vertex,
        }, math::transform::Transform, scene::{ObjectHandle, Scene, camera::Camera, entity::Object},
    }, glam::{Quat, Vec3, Vec4},
};

impl AssetManager {
    pub fn load_gltf(&mut self, path: &str) -> anyhow::Result<Scene> {
        self.reset();

        let (document, buffers, images) = gltf::import(path)?;

        let mut textures = Vec::<TextureHandle>::new();
        for image in images {
            let texture = Texture::new(image.width, image.height, image.pixels);
            let handle = TextureHandle(self.textures.len() as u32);
            self.textures.push(texture);
            textures.push(handle);
        }

        let mut materials = Vec::<MaterialHandle>::new();
        for material in document.materials() {
            let pbr = material.pbr_metallic_roughness();

            let base_color_factor = Vec4::from(pbr.base_color_factor());
            let base_color_texture = pbr
                .base_color_texture()
                .map(|t| {
                    let handle = textures[t.texture().index()];
                    self.set_color_space(handle, ColorSpace::Srgb);
                    handle
                });
            let metallic_factor = pbr.metallic_factor();
            let roughness_factor = pbr.roughness_factor();
            let metallic_roughness_texture = pbr
                .metallic_roughness_texture()
                .map(|t| {
                    let handle = textures[t.texture().index()];
                    self.set_color_space(handle, ColorSpace::Linear);
                    handle
                });
            let name = material.name().map(|s| s.to_string());
            let normal_texture = material
                .normal_texture()
                .map(|t| {
                    let handle = textures[t.texture().index()];
                    self.set_color_space(handle, ColorSpace::Linear);
                    handle
                });
            let occlusion_texture = material
                .occlusion_texture()
                .map(|t| {
                    let handle = textures[t.texture().index()];
                    self.set_color_space(handle, ColorSpace::Linear);
                    handle
                });
            let emissive_factor = Vec3::from(material.emissive_factor());
            let emissive_texture = material
                .emissive_texture()
                .map(|t| {
                    let handle = textures[t.texture().index()];
                    self.set_color_space(handle, ColorSpace::Srgb);
                    handle
                });

            let material = Material {
                base_color_factor,
                base_color_texture,
                metallic_factor,
                roughness_factor,
                metallic_roughness_texture,
                name,
                normal_texture,
                occlusion_texture,
                emissive_factor,
                emissive_texture,
            };

            let handle = MaterialHandle(self.materials.len() as u32);
            self.materials.push(material);
            materials.push(handle);
        }

        let mut meshes = Vec::<MeshHandle>::new();
        for mesh in document.meshes() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                let positions = reader.read_positions().unwrap();
                let normals = reader.read_normals().unwrap();
                let texcoords = reader.read_tex_coords(0).unwrap().into_f32();
                let tangents = reader.read_tangents().unwrap();

                let indices: Vec<u16> = reader
                    .read_indices()
                    .unwrap()
                    .into_u32()
                    .map(|i| i as u16)
                    .collect();

                let vertices: Vec<Vertex> = positions
                    .zip(normals)
                    .zip(texcoords)
                    .zip(tangents)
                    .map(|(((position, normal), uv), tangent)| Vertex {
                        position,
                        normal,
                        uv,
                        tangent,
                    })
                    .collect();

                let material = primitive
                    .material()
                    .index()
                    .map(|i| MaterialHandle(i as u32))
                    .unwrap_or(self.default_material());

                let mesh = Mesh {
                    vertices,
                    indices,
                    material,
                };

                let handle = MeshHandle(self.meshes.len() as u32);
                self.meshes.push(mesh);
                meshes.push(handle);
            }
        }

        let mut objects = Vec::<Object>::new();
        for node in document.nodes() {
            let transform_components = node.transform().decomposed();

            let position = Vec3::from(transform_components.0);
            let rotation = Quat::from_array(transform_components.1);
            let scale = Vec3::from(transform_components.2);

            let transform = Transform {
                position,
                rotation,
                scale,
            };

            let mesh = node.mesh().map(|mesh| meshes[mesh.index()].clone());

            let children: Vec<ObjectHandle> = node
                .children()
                .map(|child| ObjectHandle(child.index() as u32))
                .collect();

            objects.push(Object {
                transform,
                mesh,
                children,
                resource_handle: ObjectHandle(objects.len() as u32),
            });
        }

        let mut camera = Camera::new(
            Transform::default(),
            1.0,
            90_f32.to_radians(),
            0.1,
            100.0,
            8.0,
            180_f32.to_radians(),
        );
        camera.transform.position.z = 3.0;

        Ok(Scene { objects, camera })
    }
}
