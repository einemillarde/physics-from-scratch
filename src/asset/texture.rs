#[derive(Clone, Debug)]
pub struct Texture {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let bytes = &std::fs::read(path)?[..];
        Self::from_bytes(bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        let img = image::load_from_memory(bytes)?;

        Ok(Self {
            pixels: img.clone().into_bytes(),
            width: img.width(),
            height: img.height(),
        })
    }
}
