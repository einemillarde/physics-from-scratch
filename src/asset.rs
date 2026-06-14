pub mod material;
pub mod mesh;
pub mod preset;
pub mod texture;

use {crate::asset::texture::Texture, std::collections::HashMap};

pub struct AssetManager {
    pub textures: HashMap<String, Texture>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn load_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        path: String,
    ) -> anyhow::Result<()> {
        if let Some(_) = self.textures.get(&path) {
            return Ok(());
        }

        let texture = Texture::from_bytes(device, queue, bytes).unwrap();

        self.textures.insert(path, texture);

        Ok(())
    }
}
