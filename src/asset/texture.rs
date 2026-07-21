#[derive(Clone, Debug)]
pub struct Texture {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub color_space: Option<ColorSpace>,
}

#[derive(Clone, Debug)]
pub enum ColorSpace {
    Linear,
    Srgb,
}

impl Texture {
    pub fn new(width: u32, height: u32, pixels: Vec<u8>) -> Self {
        Self {
            width,
            height,
            pixels,
            color_space: None,
        }
    }

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
            color_space: None,
        })
    }

    pub fn set_color_space(&mut self, color_space: ColorSpace) {
        self.color_space = Some(color_space);
    }
}
