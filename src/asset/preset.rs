use crate::asset::AssetManager;

impl AssetManager {
    pub fn load_default_assets(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let mut asset_manager = AssetManager::new();

        let _ = asset_manager
            .load_texture(
                device,
                queue,
                include_bytes!("../../assets/crate.jpg"),
                "crate.jpg".to_string(),
            )
            .unwrap();

        asset_manager
    }
}
